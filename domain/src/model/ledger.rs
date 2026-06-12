//! Financial ledger domain types.
//!
//! These are pure domain models — they know nothing about TigerBeetle or any
//! other infrastructure. The adapter in `backend/src/infrastructure/tigerbeetle/`
//! maps between these and the TB wire types.

use serde::{Deserialize, Serialize};

/// Unique identifier for a ledger account.
pub type AccountId = u128;

/// Unique identifier for a ledger transfer.
pub type TransferId = u128;

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

/// A financial account managed by the ledger.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LedgerAccount {
	pub id: AccountId,
	pub debits_pending: u128,
	pub debits_posted: u128,
	pub credits_pending: u128,
	pub credits_posted: u128,
	pub ledger: u32,
	pub code: u16,
	pub flags: AccountFlags,
	pub timestamp: u64,
}

/// Data needed to create a new ledger account. The server assigns `id` and
/// `timestamp`; balance fields start at zero.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewLedgerAccount {
	pub id: AccountId,
	pub ledger: u32,
	pub code: u16,
	pub flags: AccountFlags,
}

/// A double-entry transfer between two accounts.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LedgerTransfer {
	pub id: TransferId,
	pub debit_account_id: AccountId,
	pub credit_account_id: AccountId,
	pub amount: u128,
	pub pending_id: Option<TransferId>,
	pub ledger: u32,
	pub code: u16,
	pub flags: TransferFlags,
	pub timestamp: u64,
}

/// Data needed to create a new transfer. The server assigns `timestamp`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewLedgerTransfer {
	pub id: TransferId,
	pub debit_account_id: AccountId,
	pub credit_account_id: AccountId,
	pub amount: u128,
	pub pending_id: Option<TransferId>,
	pub ledger: u32,
	pub code: u16,
	pub flags: TransferFlags,
}

/// Historical balance snapshot for a point in time.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountBalance {
	pub debits_pending: u128,
	pub debits_posted: u128,
	pub credits_pending: u128,
	pub credits_posted: u128,
	pub timestamp: u64,
}
