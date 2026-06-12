//! TigerBeetle adapter implementing the [`Ledger`] port.
//!
//! Maps between domain-level financial types and the TigerBeetle wire protocol
//! via the official `tigerbeetle` Rust client. Holds a single `Client` instance
//! — the client is internally synchronised and safe to share across tasks.

use async_trait::async_trait;
use domain::{
	error::DomainError,
	model::ledger::{AccountBalance, AccountFlags, AccountId, LedgerAccount, LedgerTransfer, NewLedgerAccount, NewLedgerTransfer, TransferFlags, TransferId},
};
use tigerbeetle as tb;

use crate::domain::port::ledger::Ledger;

/// Default max batch size for the TigerBeetle server.
const BATCH_SIZE: usize = 8189;

/// TigerBeetle-backed adapter implementing the [`Ledger`] port.
pub struct TigerBeetleLedger {
	client: tb::Client,
}

impl TigerBeetleLedger {
	/// Create a new TigerBeetle-backed ledger adapter.
	///
	/// `cluster_id` is the TigerBeetle cluster identifier (use `0` for
	/// single-node dev). `address` is the replica address in any format
	/// accepted by `tb::Client::new` (`"3000"`, `"127.0.0.1:3000"`, etc.).
	pub fn try_new(cluster_id: u128, address: &str) -> Result<Self, DomainError> {
		let client = tb::Client::new(cluster_id, address).map_err(map_init_error)?;
		Ok(Self { client })
	}
}

// ── Domain ↔ TB mapping ────────────────────────────────────────────────────

fn account_flags_to_tb(flags: AccountFlags) -> tb::AccountFlags {
	let mut tb_flags = tb::AccountFlags::empty();
	if flags.contains(AccountFlags::LINKED) {
		tb_flags |= tb::AccountFlags::Linked;
	}
	if flags.contains(AccountFlags::DEBITS_MUST_NOT_EXCEED_CREDITS) {
		tb_flags |= tb::AccountFlags::DebitsMustNotExceedCredits;
	}
	if flags.contains(AccountFlags::CREDITS_MUST_NOT_EXCEED_DEBITS) {
		tb_flags |= tb::AccountFlags::CreditsMustNotExceedDebits;
	}
	if flags.contains(AccountFlags::HISTORY) {
		tb_flags |= tb::AccountFlags::History;
	}
	tb_flags
}

fn account_flags_from_tb(flags: tb::AccountFlags) -> AccountFlags {
	let mut acc_flags = AccountFlags::NONE;
	if flags.contains(tb::AccountFlags::Linked) {
		acc_flags = acc_flags | AccountFlags::LINKED;
	}
	if flags.contains(tb::AccountFlags::DebitsMustNotExceedCredits) {
		acc_flags = acc_flags | AccountFlags::DEBITS_MUST_NOT_EXCEED_CREDITS;
	}
	if flags.contains(tb::AccountFlags::CreditsMustNotExceedDebits) {
		acc_flags = acc_flags | AccountFlags::CREDITS_MUST_NOT_EXCEED_DEBITS;
	}
	if flags.contains(tb::AccountFlags::History) {
		acc_flags = acc_flags | AccountFlags::HISTORY;
	}
	acc_flags
}

fn transfer_flags_to_tb(flags: TransferFlags) -> tb::TransferFlags {
	let mut tb_flags = tb::TransferFlags::empty();
	if flags.contains(TransferFlags::LINKED) {
		tb_flags |= tb::TransferFlags::Linked;
	}
	if flags.contains(TransferFlags::PENDING) {
		tb_flags |= tb::TransferFlags::Pending;
	}
	if flags.contains(TransferFlags::POST_PENDING) {
		tb_flags |= tb::TransferFlags::PostPendingTransfer;
	}
	if flags.contains(TransferFlags::VOID_PENDING) {
		tb_flags |= tb::TransferFlags::VoidPendingTransfer;
	}
	tb_flags
}

fn transfer_flags_from_tb(flags: tb::TransferFlags) -> TransferFlags {
	let mut tr_flags = TransferFlags::NONE;
	if flags.contains(tb::TransferFlags::Linked) {
		tr_flags = tr_flags | TransferFlags::LINKED;
	}
	if flags.contains(tb::TransferFlags::Pending) {
		tr_flags = tr_flags | TransferFlags::PENDING;
	}
	if flags.contains(tb::TransferFlags::PostPendingTransfer) {
		tr_flags = tr_flags | TransferFlags::POST_PENDING;
	}
	if flags.contains(tb::TransferFlags::VoidPendingTransfer) {
		tr_flags = tr_flags | TransferFlags::VOID_PENDING;
	}
	tr_flags
}

fn to_tb_account(a: &NewLedgerAccount) -> tb::Account {
	tb::Account {
		id: a.id,
		ledger: a.ledger,
		code: a.code,
		flags: account_flags_to_tb(a.flags),
		..Default::default()
	}
}

fn from_tb_account(a: tb::Account) -> LedgerAccount {
	LedgerAccount {
		id: a.id,
		debits_pending: a.debits_pending,
		debits_posted: a.debits_posted,
		credits_pending: a.credits_pending,
		credits_posted: a.credits_posted,
		ledger: a.ledger,
		code: a.code,
		flags: account_flags_from_tb(a.flags),
		timestamp: a.timestamp,
	}
}

fn to_tb_transfer(t: &NewLedgerTransfer) -> tb::Transfer {
	// TB uses id=0 to mean "no pending transfer"; the domain uses Option.
	// TB itself rejects id=0 at creation, so the sentinel round-trips safely.
	tb::Transfer {
		id: t.id,
		debit_account_id: t.debit_account_id,
		credit_account_id: t.credit_account_id,
		amount: t.amount,
		pending_id: t.pending_id.unwrap_or(0),
		ledger: t.ledger,
		code: t.code,
		flags: transfer_flags_to_tb(t.flags),
		..Default::default()
	}
}

fn from_tb_transfer(t: tb::Transfer) -> LedgerTransfer {
	LedgerTransfer {
		id: t.id,
		debit_account_id: t.debit_account_id,
		credit_account_id: t.credit_account_id,
		amount: t.amount,
		// TB uses id=0 to mean "no pending transfer".
		pending_id: if t.pending_id == 0 { None } else { Some(t.pending_id) },
		ledger: t.ledger,
		code: t.code,
		flags: transfer_flags_from_tb(t.flags),
		timestamp: t.timestamp,
	}
}

fn from_tb_balance(b: tb::AccountBalance) -> AccountBalance {
	AccountBalance {
		debits_pending: b.debits_pending,
		debits_posted: b.debits_posted,
		credits_pending: b.credits_pending,
		credits_posted: b.credits_posted,
		timestamp: b.timestamp,
	}
}

// ── Ledger port implementation ──────────────────────────────────────────────

#[async_trait]
impl Ledger for TigerBeetleLedger {
	/// Creates accounts in batches. TB's `create_accounts` returns status +
	/// timestamps but NOT the full account state (balances), so we must
	/// issue a follow-up lookup to get the server-assigned state.
	///
	/// TB is idempotent: retrying the same batch is safe — items already
	/// created return `Exists`. Partial failures across chunks leave earlier
	/// chunks committed, which is by design: a retry of the full set will
	/// find those as `Exists`.
	async fn create_accounts(&self, accounts: &[NewLedgerAccount]) -> Result<Vec<LedgerAccount>, DomainError> {
		if accounts.is_empty() {
			return Ok(Vec::new());
		}

		let tb_accounts: Vec<tb::Account> = accounts.iter().map(to_tb_account).collect();

		for chunk in tb_accounts.chunks(BATCH_SIZE) {
			let results = self
				.client
				.create_accounts(chunk)
				.map_err(map_closed_error)?
				.await
				.map_err(|e| map_packet_error("create_accounts", e))?;
			for result in &results {
				match result.status {
					tb::CreateAccountStatus::Created | tb::CreateAccountStatus::Exists => {}
					_ => return Err(map_create_account_error(result.status)),
				}
			}
		}

		// Look up to return full state with server-assigned balances.
		let ids: Vec<u128> = accounts.iter().map(|a| a.id).collect();
		let looked_up = self
			.client
			.lookup_accounts(&ids)
			.map_err(map_closed_error)?
			.await
			.map_err(|e| map_packet_error("lookup_accounts", e))?;
		Ok(looked_up.into_iter().map(from_tb_account).collect())
	}

	async fn lookup_accounts(&self, ids: &[AccountId]) -> Result<Vec<LedgerAccount>, DomainError> {
		if ids.is_empty() {
			return Ok(Vec::new());
		}

		let results = self
			.client
			.lookup_accounts(ids)
			.map_err(map_closed_error)?
			.await
			.map_err(|e| map_packet_error("lookup_accounts", e))?;

		Ok(results.into_iter().map(from_tb_account).collect())
	}

	async fn create_transfers(&self, transfers: &[NewLedgerTransfer]) -> Result<Vec<LedgerTransfer>, DomainError> {
		if transfers.is_empty() {
			return Ok(Vec::new());
		}

		let tb_transfers: Vec<tb::Transfer> = transfers.iter().map(to_tb_transfer).collect();

		for chunk in tb_transfers.chunks(BATCH_SIZE) {
			let results = self
				.client
				.create_transfers(chunk)
				.map_err(map_closed_error)?
				.await
				.map_err(|e| map_packet_error("create_transfers", e))?;
			for result in &results {
				match result.status {
					tb::CreateTransferStatus::Created | tb::CreateTransferStatus::Exists => {}
					_ => return Err(map_create_transfer_error(result.status)),
				}
			}
		}

		// Look up to return full state with server-assigned timestamps.
		let ids: Vec<u128> = transfers.iter().map(|t| t.id).collect();
		let looked_up = self
			.client
			.lookup_transfers(&ids)
			.map_err(map_closed_error)?
			.await
			.map_err(|e| map_packet_error("lookup_transfers", e))?;
		Ok(looked_up.into_iter().map(from_tb_transfer).collect())
	}

	async fn lookup_transfers(&self, ids: &[TransferId]) -> Result<Vec<LedgerTransfer>, DomainError> {
		if ids.is_empty() {
			return Ok(Vec::new());
		}

		let results = self
			.client
			.lookup_transfers(ids)
			.map_err(map_closed_error)?
			.await
			.map_err(|e| map_packet_error("lookup_transfers", e))?;

		Ok(results.into_iter().map(from_tb_transfer).collect())
	}

	async fn get_account_balances(&self, account_id: AccountId) -> Result<Vec<AccountBalance>, DomainError> {
		let filter = tb::AccountFilter { account_id, ..Default::default() };
		let results = self
			.client
			.get_account_balances(filter)
			.map_err(map_closed_error)?
			.await
			.map_err(|e| map_packet_error("get_account_balances", e))?;

		Ok(results.into_iter().map(from_tb_balance).collect())
	}
}

// ── Error mapping ───────────────────────────────────────────────────────────

fn map_init_error(err: tb::InitStatus) -> DomainError {
	DomainError::Repository(format!("tigerbeetle client init failed: {err:?}"))
}

fn map_closed_error(_: tb::ClientClosed) -> DomainError {
	DomainError::Repository("tigerbeetle client is closed".into())
}

fn map_packet_error(op: &str, err: tb::PacketStatus) -> DomainError {
	DomainError::Repository(format!("tigerbeetle {op} request failed: {err:?}"))
}

fn map_create_account_error(status: tb::CreateAccountStatus) -> DomainError {
	match status {
		// Conflict
		tb::CreateAccountStatus::LinkedEventFailed => DomainError::Conflict("linked account event failed".into()),
		tb::CreateAccountStatus::ExistsWithDifferentFlags
		| tb::CreateAccountStatus::ExistsWithDifferentUserData128
		| tb::CreateAccountStatus::ExistsWithDifferentUserData64
		| tb::CreateAccountStatus::ExistsWithDifferentUserData32
		| tb::CreateAccountStatus::ExistsWithDifferentLedger
		| tb::CreateAccountStatus::ExistsWithDifferentCode => DomainError::Conflict("account exists with different details".into()),
		// Validation
		tb::CreateAccountStatus::FlagsAreMutuallyExclusive => DomainError::Validation("account flags are mutually exclusive".into()),
		tb::CreateAccountStatus::IdMustNotBeZero | tb::CreateAccountStatus::IdMustNotBeIntMax => DomainError::Validation("invalid account id".into()),
		tb::CreateAccountStatus::LedgerMustNotBeZero => DomainError::Validation("account ledger must not be zero".into()),
		tb::CreateAccountStatus::CodeMustNotBeZero => DomainError::Validation("account code must not be zero".into()),
		tb::CreateAccountStatus::DebitsPendingMustBeZero
		| tb::CreateAccountStatus::DebitsPostedMustBeZero
		| tb::CreateAccountStatus::CreditsPendingMustBeZero
		| tb::CreateAccountStatus::CreditsPostedMustBeZero => DomainError::Validation("account balance fields must be zero on creation".into()),
		// Everything else is an unexpected infrastructure result.
		_ => DomainError::Repository(format!("unexpected account creation result: {status:?}")),
	}
}

fn map_create_transfer_error(status: tb::CreateTransferStatus) -> DomainError {
	match status {
		// NotFound
		tb::CreateTransferStatus::DebitAccountNotFound => DomainError::NotFound {
			entity: "debit account",
			id: "unknown".into(),
		},
		tb::CreateTransferStatus::CreditAccountNotFound => DomainError::NotFound {
			entity: "credit account",
			id: "unknown".into(),
		},
		tb::CreateTransferStatus::PendingTransferNotFound => DomainError::NotFound {
			entity: "pending transfer",
			id: "unknown".into(),
		},
		// Conflict
		tb::CreateTransferStatus::LinkedEventFailed => DomainError::Conflict("linked transfer event failed".into()),
		tb::CreateTransferStatus::ExistsWithDifferentFlags
		| tb::CreateTransferStatus::ExistsWithDifferentPendingId
		| tb::CreateTransferStatus::ExistsWithDifferentTimeout
		| tb::CreateTransferStatus::ExistsWithDifferentDebitAccountId
		| tb::CreateTransferStatus::ExistsWithDifferentCreditAccountId
		| tb::CreateTransferStatus::ExistsWithDifferentAmount
		| tb::CreateTransferStatus::ExistsWithDifferentUserData128
		| tb::CreateTransferStatus::ExistsWithDifferentUserData64
		| tb::CreateTransferStatus::ExistsWithDifferentUserData32
		| tb::CreateTransferStatus::ExistsWithDifferentLedger
		| tb::CreateTransferStatus::ExistsWithDifferentCode => DomainError::Conflict("transfer exists with different details".into()),
		tb::CreateTransferStatus::ExceedsCredits
		| tb::CreateTransferStatus::ExceedsDebits
		| tb::CreateTransferStatus::ExceedsPendingTransferAmount
		| tb::CreateTransferStatus::OverflowsDebits
		| tb::CreateTransferStatus::OverflowsCredits
		| tb::CreateTransferStatus::OverflowsDebitsPending
		| tb::CreateTransferStatus::OverflowsCreditsPending
		| tb::CreateTransferStatus::OverflowsDebitsPosted
		| tb::CreateTransferStatus::OverflowsCreditsPosted
		| tb::CreateTransferStatus::OverflowsTimeout => DomainError::Conflict("transfer exceeds account limits".into()),
		tb::CreateTransferStatus::DebitAccountAlreadyClosed | tb::CreateTransferStatus::CreditAccountAlreadyClosed => DomainError::Conflict("account is closed".into()),
		tb::CreateTransferStatus::PendingTransferAlreadyPosted | tb::CreateTransferStatus::PendingTransferAlreadyVoided | tb::CreateTransferStatus::PendingTransferExpired =>
			DomainError::Conflict("pending transfer no longer valid".into()),
		tb::CreateTransferStatus::PendingTransferNotPending => DomainError::Conflict("transfer referenced as pending is not pending".into()),
		tb::CreateTransferStatus::PendingTransferHasDifferentDebitAccountId
		| tb::CreateTransferStatus::PendingTransferHasDifferentCreditAccountId
		| tb::CreateTransferStatus::PendingTransferHasDifferentLedger
		| tb::CreateTransferStatus::PendingTransferHasDifferentCode
		| tb::CreateTransferStatus::PendingTransferHasDifferentAmount => DomainError::Conflict("pending transfer details mismatch".into()),
		tb::CreateTransferStatus::IdAlreadyFailed => DomainError::Conflict("transfer id has already failed".into()),
		tb::CreateTransferStatus::ImportedEventTimestampMustNotRegress
		| tb::CreateTransferStatus::ImportedEventTimestampMustPostdateDebitAccount
		| tb::CreateTransferStatus::ImportedEventTimestampMustPostdateCreditAccount
		| tb::CreateTransferStatus::ImportedEventTimeoutMustBeZero => DomainError::Conflict("imported event timestamp violation".into()),
		// Validation
		tb::CreateTransferStatus::FlagsAreMutuallyExclusive => DomainError::Validation("transfer flags are mutually exclusive".into()),
		tb::CreateTransferStatus::IdMustNotBeZero | tb::CreateTransferStatus::IdMustNotBeIntMax => DomainError::Validation("invalid transfer id".into()),
		tb::CreateTransferStatus::DebitAccountIdMustNotBeZero
		| tb::CreateTransferStatus::DebitAccountIdMustNotBeIntMax
		| tb::CreateTransferStatus::CreditAccountIdMustNotBeZero
		| tb::CreateTransferStatus::CreditAccountIdMustNotBeIntMax => DomainError::Validation("account id must be a valid u128".into()),
		tb::CreateTransferStatus::AccountsMustBeDifferent => DomainError::Validation("debit and credit accounts must be different".into()),
		tb::CreateTransferStatus::AccountsMustHaveTheSameLedger | tb::CreateTransferStatus::TransferMustHaveTheSameLedgerAsAccounts =>
			DomainError::Validation("transfer and accounts must share the same ledger".into()),
		tb::CreateTransferStatus::PendingIdMustNotBeZero | tb::CreateTransferStatus::PendingIdMustNotBeIntMax | tb::CreateTransferStatus::PendingIdMustBeDifferent =>
			DomainError::Validation("invalid pending transfer id".into()),
		tb::CreateTransferStatus::TimeoutReservedForPendingTransfer => DomainError::Validation("timeout is reserved for pending transfers only".into()),
		tb::CreateTransferStatus::ClosingTransferMustBePending => DomainError::Validation("closing transfer must reference a pending transfer".into()),
		tb::CreateTransferStatus::LedgerMustNotBeZero => DomainError::Validation("transfer ledger must not be zero".into()),
		tb::CreateTransferStatus::CodeMustNotBeZero => DomainError::Validation("transfer code must not be zero".into()),
		// Everything else is an unexpected infrastructure result.
		_ => DomainError::Repository(format!("unexpected transfer result: {status:?}")),
	}
}
