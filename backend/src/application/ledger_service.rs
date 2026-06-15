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

/// Ubiquitous-language names for the ledger read models, used in
/// `NotFound { entity, .. }`. The ledger types are reached through a [`Ledger`]
/// gateway, not a `Repository`, so they implement no `AggregateRoot` and carry
/// no `NAME` const to borrow — these keep the lookup sites from drifting.
const ACCOUNT: &str = "account";
const TRANSFER: &str = "transfer";

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
			entity: ACCOUNT,
			id: id.to_string(),
		})
	}

	pub async fn create_transfers(&self, transfers: Vec<NewLedgerTransfer>) -> Result<Vec<LedgerTransfer>, DomainError> {
		self.ledger.create_transfers(&transfers).await
	}

	pub async fn get_transfer(&self, id: TransferId) -> Result<LedgerTransfer, DomainError> {
		let results = self.ledger.lookup_transfers(&[id]).await?;
		results.into_iter().next().ok_or(DomainError::NotFound {
			entity: TRANSFER,
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
	use domain::{
		architecture::Gateway,
		model::ledger::{AccountFlags, Amount, Code, LedgerCode, TransferFlags},
	};

	use super::*;

	/// In-memory fake of the [`Ledger`] port for unit-testing services.
	#[derive(Default)]
	struct InMemoryLedger {
		accounts: Mutex<Vec<LedgerAccount>>,
		transfers: Mutex<Vec<LedgerTransfer>>,
	}

	impl Gateway for InMemoryLedger {}

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
			id: AccountId::from_raw(1),
			ledger: LedgerCode::parse(1).unwrap(),
			code: Code::parse(1).unwrap(),
			flags: AccountFlags::HISTORY,
		}
	}

	#[tokio::test]
	async fn create_and_lookup_account_roundtrips() {
		let svc = service();
		let created = svc.create_accounts(vec![sample_account()]).await.unwrap();
		assert_eq!(created.len(), 1);
		assert_eq!(created[0].id, AccountId::from_raw(1));

		let fetched = svc.get_account(AccountId::from_raw(1)).await.unwrap();
		assert_eq!(fetched.id, AccountId::from_raw(1));
		assert_eq!(fetched.code.get(), 1);
	}

	#[tokio::test]
	async fn get_missing_account_returns_not_found() {
		let svc = service();
		let err = svc.get_account(AccountId::from_raw(999)).await.unwrap_err();
		assert!(matches!(err, DomainError::NotFound { entity: "account", .. }));
	}

	#[tokio::test]
	async fn create_and_lookup_transfer_roundtrips() {
		let svc = service();
		// First create the accounts.
		svc.create_accounts(vec![
			NewLedgerAccount {
				id: AccountId::from_raw(1),
				ledger: LedgerCode::parse(1).unwrap(),
				code: Code::parse(1).unwrap(),
				flags: AccountFlags::NONE,
			},
			NewLedgerAccount {
				id: AccountId::from_raw(2),
				ledger: LedgerCode::parse(1).unwrap(),
				code: Code::parse(1).unwrap(),
				flags: AccountFlags::NONE,
			},
		])
		.await
		.unwrap();

		let transfer = NewLedgerTransfer {
			id: TransferId::from_raw(100),
			debit_account_id: AccountId::from_raw(1),
			credit_account_id: AccountId::from_raw(2),
			amount: Amount::new(42),
			pending_id: None,
			ledger: LedgerCode::parse(1).unwrap(),
			code: Code::parse(1).unwrap(),
			flags: TransferFlags::NONE,
		};
		let created = svc.create_transfers(vec![transfer]).await.unwrap();
		assert_eq!(created.len(), 1);
		assert_eq!(created[0].amount.get(), 42);

		let fetched = svc.get_transfer(TransferId::from_raw(100)).await.unwrap();
		assert_eq!(fetched.amount.get(), 42);
	}

	#[tokio::test]
	async fn get_missing_transfer_returns_not_found() {
		let svc = service();
		let err = svc.get_transfer(TransferId::from_raw(999)).await.unwrap_err();
		assert!(matches!(err, DomainError::NotFound { entity: "transfer", .. }));
	}
}
