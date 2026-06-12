//! Use cases operating on the financial ledger.
//!
//! Depends only on the [`Ledger`] port, so it is agnostic to the concrete
//! adapter and trivially testable with an in-memory fake.

use std::sync::Arc;

use domain::{
	error::DomainError,
	model::ledger::{AccountBalance, AccountId, LedgerAccount, LedgerTransfer, NewLedgerAccount, NewLedgerTransfer, TransferId},
};

use crate::domain::port::ledger::Ledger;

#[derive(Clone)]
pub struct LedgerService {
	ledger: Arc<dyn Ledger>,
}

impl LedgerService {
	pub fn new(ledger: Arc<dyn Ledger>) -> Self {
		Self { ledger }
	}

	pub async fn create_accounts(&self, accounts: Vec<NewLedgerAccount>) -> Result<Vec<LedgerAccount>, DomainError> {
		self.ledger.create_accounts(&accounts).await
	}

	pub async fn get_account(&self, id: AccountId) -> Result<LedgerAccount, DomainError> {
		let results = self.ledger.lookup_accounts(&[id]).await?;
		results.into_iter().next().ok_or(DomainError::NotFound {
			entity: "account",
			id: id.to_string(),
		})
	}

	pub async fn create_transfers(&self, transfers: Vec<NewLedgerTransfer>) -> Result<Vec<LedgerTransfer>, DomainError> {
		self.ledger.create_transfers(&transfers).await
	}

	pub async fn get_transfer(&self, id: TransferId) -> Result<LedgerTransfer, DomainError> {
		let results = self.ledger.lookup_transfers(&[id]).await?;
		results.into_iter().next().ok_or(DomainError::NotFound {
			entity: "transfer",
			id: id.to_string(),
		})
	}

	pub async fn get_account_balances(&self, account_id: AccountId) -> Result<Vec<AccountBalance>, DomainError> {
		self.ledger.get_account_balances(account_id).await
	}
}

#[cfg(test)]
mod tests {
	use std::sync::{Arc, Mutex};

	use async_trait::async_trait;
	use domain::model::ledger::{AccountFlags, TransferFlags};

	use super::*;

	/// In-memory fake of the [`Ledger`] port for unit-testing services.
	#[derive(Default)]
	struct InMemoryLedger {
		accounts: Mutex<Vec<LedgerAccount>>,
		transfers: Mutex<Vec<LedgerTransfer>>,
	}

	#[async_trait]
	impl Ledger for InMemoryLedger {
		async fn create_accounts(&self, accounts: &[NewLedgerAccount]) -> Result<Vec<LedgerAccount>, DomainError> {
			let mut stored = self.accounts.lock().unwrap();
			let created: Vec<LedgerAccount> = accounts
				.iter()
				.map(|a| LedgerAccount {
					id: a.id,
					debits_pending: 0,
					debits_posted: 0,
					credits_pending: 0,
					credits_posted: 0,
					ledger: a.ledger,
					code: a.code,
					flags: a.flags,
					timestamp: 1,
				})
				.collect();
			stored.extend(created.clone());
			Ok(created)
		}

		async fn lookup_accounts(&self, ids: &[AccountId]) -> Result<Vec<LedgerAccount>, DomainError> {
			let stored = self.accounts.lock().unwrap();
			Ok(stored.iter().filter(|a| ids.contains(&a.id)).cloned().collect())
		}

		async fn create_transfers(&self, transfers: &[NewLedgerTransfer]) -> Result<Vec<LedgerTransfer>, DomainError> {
			let mut stored = self.transfers.lock().unwrap();
			let created: Vec<LedgerTransfer> = transfers
				.iter()
				.map(|t| LedgerTransfer {
					id: t.id,
					debit_account_id: t.debit_account_id,
					credit_account_id: t.credit_account_id,
					amount: t.amount,
					pending_id: t.pending_id,
					ledger: t.ledger,
					code: t.code,
					flags: t.flags,
					timestamp: 1,
				})
				.collect();
			stored.extend(created.clone());
			Ok(created)
		}

		async fn lookup_transfers(&self, ids: &[TransferId]) -> Result<Vec<LedgerTransfer>, DomainError> {
			let stored = self.transfers.lock().unwrap();
			Ok(stored.iter().filter(|t| ids.contains(&t.id)).cloned().collect())
		}

		async fn get_account_balances(&self, _account_id: AccountId) -> Result<Vec<AccountBalance>, DomainError> {
			Ok(Vec::new())
		}
	}

	fn service() -> LedgerService {
		LedgerService::new(Arc::new(InMemoryLedger::default()))
	}

	fn sample_account() -> NewLedgerAccount {
		NewLedgerAccount {
			id: 1,
			ledger: 1,
			code: 1,
			flags: AccountFlags::HISTORY,
		}
	}

	#[tokio::test]
	async fn create_and_lookup_account_roundtrips() {
		let svc = service();
		let created = svc.create_accounts(vec![sample_account()]).await.unwrap();
		assert_eq!(created.len(), 1);
		assert_eq!(created[0].id, 1);

		let fetched = svc.get_account(1).await.unwrap();
		assert_eq!(fetched.id, 1);
		assert_eq!(fetched.code, 1);
	}

	#[tokio::test]
	async fn get_missing_account_returns_not_found() {
		let svc = service();
		let err = svc.get_account(999).await.unwrap_err();
		assert!(matches!(err, DomainError::NotFound { entity: "account", .. }));
	}

	#[tokio::test]
	async fn create_and_lookup_transfer_roundtrips() {
		let svc = service();
		// First create the accounts.
		svc.create_accounts(vec![
			NewLedgerAccount {
				id: 1,
				ledger: 1,
				code: 1,
				flags: AccountFlags::NONE,
			},
			NewLedgerAccount {
				id: 2,
				ledger: 1,
				code: 1,
				flags: AccountFlags::NONE,
			},
		])
		.await
		.unwrap();

		let transfer = NewLedgerTransfer {
			id: 100,
			debit_account_id: 1,
			credit_account_id: 2,
			amount: 42,
			pending_id: None,
			ledger: 1,
			code: 1,
			flags: TransferFlags::NONE,
		};
		let created = svc.create_transfers(vec![transfer]).await.unwrap();
		assert_eq!(created.len(), 1);
		assert_eq!(created[0].amount, 42);

		let fetched = svc.get_transfer(100).await.unwrap();
		assert_eq!(fetched.amount, 42);
	}

	#[tokio::test]
	async fn get_missing_transfer_returns_not_found() {
		let svc = service();
		let err = svc.get_transfer(999).await.unwrap_err();
		assert!(matches!(err, DomainError::NotFound { entity: "transfer", .. }));
	}
}
