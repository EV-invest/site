//! Financial ledger domain types.
//!
//! These are pure domain models — they know nothing about TigerBeetle or any
//! other infrastructure. The adapter in `backend/src/infrastructure/tigerbeetle/`
//! maps between these and the TB wire types.
//!
//! TigerBeetle is this ledger's consistency boundary: double-entry and two-phase
//! (pending/post/void) invariants are enforced by TB and surfaced as
//! `DomainError` by the adapter — do not re-implement them here.

use serde::{Deserialize, Serialize};

use crate::{architecture::Id, error::DomainError};

/// Unique identifier for a ledger account.
pub type AccountId = Id<AccountTag, u128>;
/// Unique identifier for a ledger transfer.
pub type TransferId = Id<TransferTag, u128>;
/// Tag types giving each ledger identity its own incompatible newtype, so an
/// `AccountId` can never be passed where a `TransferId` is expected.
///
/// These give the ledger types typed identities, but `LedgerAccount` and
/// `LedgerTransfer` deliberately implement neither [`Entity`](crate::architecture::Entity)
/// nor [`AggregateRoot`](crate::architecture::AggregateRoot): those mark a *local*
/// consistency boundary owned by a `Repository`, whereas the ledger is reached
/// through a `Gateway` (TigerBeetle owns identity, invariants, and atomicity).
/// Modeling them as aggregate roots would falsely imply a Postgres-side
/// repository and a local transaction that does not — and must not — exist.
pub struct AccountTag;
pub struct TransferTag;



/// A monetary amount. Newtype only — no zero rejection: TigerBeetle permits
/// `amount = 0` (e.g. balancing transfers) and is the authority, so a local
/// `> 0` rule would diverge and emit a false validation error. The type-safety
/// win is keeping money out of bare `u128` arithmetic at the boundary.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Amount(u128);

impl Amount {
	pub const fn new(value: u128) -> Self {
		Self(value)
	}

	pub const fn get(self) -> u128 {
		self.0
	}
}

/// The ledger number an account/transfer belongs to. TigerBeetle rejects `0`
/// unconditionally, so this newtype rejects it too — turning a server round-trip
/// rejection into a local validation error with no divergence risk.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct LedgerCode(u32);

impl LedgerCode {
	pub fn parse(value: u32) -> Result<Self, DomainError> {
		if value == 0 {
			return Err(DomainError::Validation("ledger must not be zero".into()));
		}
		Ok(Self(value))
	}

	pub const fn get(self) -> u32 {
		self.0
	}
}

/// The user-defined reason/category code for an account or transfer. As with
/// [`LedgerCode`], TigerBeetle rejects `0`, so this rejects it locally.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Code(u16);

impl Code {
	pub fn parse(value: u16) -> Result<Self, DomainError> {
		if value == 0 {
			return Err(DomainError::Validation("code must not be zero".into()));
		}
		Ok(Self(value))
	}

	pub const fn get(self) -> u16 {
		self.0
	}
}

/// Bitflags for account behaviour, mirroring the TigerBeetle `AccountFlags`
/// semantics but expressed as a domain type so the core stays I/O-free.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AccountFlags(pub u16);

impl AccountFlags {
	pub const CREDITS_MUST_NOT_EXCEED_DEBITS: Self = Self(1 << 2);
	pub const DEBITS_MUST_NOT_EXCEED_CREDITS: Self = Self(1 << 1);
	pub const HISTORY: Self = Self(1 << 3);
	pub const LINKED: Self = Self(1 << 0);
	pub const NONE: Self = Self(0);

	pub const fn contains(self, other: Self) -> bool {
		self.0 & other.0 != 0
	}

	pub const fn union(self, other: Self) -> Self {
		Self(self.0 | other.0)
	}
}

impl std::ops::BitOr for AccountFlags {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self {
		self.union(rhs)
	}
}

/// Bitflags for transfer behaviour.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TransferFlags(pub u16);

impl TransferFlags {
	pub const LINKED: Self = Self(1 << 0);
	pub const NONE: Self = Self(0);
	pub const PENDING: Self = Self(1 << 1);
	pub const POST_PENDING: Self = Self(1 << 2);
	pub const VOID_PENDING: Self = Self(1 << 3);

	pub const fn contains(self, other: Self) -> bool {
		self.0 & other.0 != 0
	}

	pub const fn union(self, other: Self) -> Self {
		Self(self.0 | other.0)
	}
}

impl std::ops::BitOr for TransferFlags {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self {
		self.union(rhs)
	}
}

/// A financial account managed by the ledger. Balance fields stay raw `u128`:
/// they are computed and owned by TigerBeetle, not domain-authored values.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LedgerAccount {
	pub id: AccountId,
	pub debits_pending: u128,
	pub debits_posted: u128,
	pub credits_pending: u128,
	pub credits_posted: u128,
	pub ledger: LedgerCode,
	pub code: Code,
	pub flags: AccountFlags,
	pub timestamp: u64,
}

/// Data needed to create a new ledger account. The server assigns `timestamp`;
/// balance fields start at zero.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewLedgerAccount {
	pub id: AccountId,
	pub ledger: LedgerCode,
	pub code: Code,
	pub flags: AccountFlags,
}

/// A double-entry transfer between two accounts.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LedgerTransfer {
	pub id: TransferId,
	pub debit_account_id: AccountId,
	pub credit_account_id: AccountId,
	pub amount: Amount,
	pub pending_id: Option<TransferId>,
	pub ledger: LedgerCode,
	pub code: Code,
	pub flags: TransferFlags,
	pub timestamp: u64,
}

/// Data needed to create a new transfer. The server assigns `timestamp`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewLedgerTransfer {
	pub id: TransferId,
	pub debit_account_id: AccountId,
	pub credit_account_id: AccountId,
	pub amount: Amount,
	pub pending_id: Option<TransferId>,
	pub ledger: LedgerCode,
	pub code: Code,
	pub flags: TransferFlags,
}

/// Historical balance snapshot for a point in time. TB-owned values stay raw.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountBalance {
	pub debits_pending: u128,
	pub debits_posted: u128,
	pub credits_pending: u128,
	pub credits_posted: u128,
	pub timestamp: u64,
}
