use async_trait::async_trait;
use domain::{
	error::DomainError,
	model::ledger::{AccountBalance, AccountId, LedgerAccount, LedgerTransfer, NewLedgerAccount, NewLedgerTransfer, TransferId},
};

/// Outbound port for the financial ledger.
///
/// Abstracts the double-entry accounting store (TigerBeetle) behind
/// domain-level operations. The core depends on this trait; concrete adapters
/// in `crate::infrastructure` implement it.
#[async_trait]
pub trait Ledger: Send + Sync {
	/// Create one or more accounts. Returns the created accounts in server
	/// order with assigned timestamps.
	async fn create_accounts(&self, accounts: &[NewLedgerAccount]) -> Result<Vec<LedgerAccount>, DomainError>;

	/// Look up accounts by ID. Returned vec may be in any order; match by
	/// `id`. Accounts that don't exist are silently omitted.
	async fn lookup_accounts(&self, ids: &[AccountId]) -> Result<Vec<LedgerAccount>, DomainError>;

	/// Create one or more transfers. Returns the created transfers in server
	/// order with assigned timestamps.
	async fn create_transfers(&self, transfers: &[NewLedgerTransfer]) -> Result<Vec<LedgerTransfer>, DomainError>;

	/// Look up transfers by ID. Transfers that don't exist are silently
	/// omitted.
	async fn lookup_transfers(&self, ids: &[TransferId]) -> Result<Vec<LedgerTransfer>, DomainError>;

	/// Fetch historical balance records for an account. Requires the
	/// `HISTORY` flag on the account.
	async fn get_account_balances(&self, account_id: AccountId) -> Result<Vec<AccountBalance>, DomainError>;
}
