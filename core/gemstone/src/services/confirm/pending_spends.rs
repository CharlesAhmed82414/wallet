use std::sync::{Mutex, MutexGuard, PoisonError};

use primitives::{Chain, Outpoint, TransactionId, WalletId};

#[derive(Debug, Clone, PartialEq)]
pub(super) struct PendingSpend {
    pub wallet_id: WalletId,
    pub transaction_id: TransactionId,
    pub address: String,
    pub outpoints: Vec<Outpoint>,
    pub recorded: bool,
}

#[derive(Default)]
pub(super) struct PendingSpends {
    spends: Mutex<Vec<PendingSpend>>,
}

impl PendingSpends {
    pub fn insert(&self, spends: Vec<PendingSpend>) {
        self.spends().extend(spends);
    }

    pub fn for_address(&self, chain: Chain, address: &str) -> Vec<PendingSpend> {
        self.spends().iter().filter(|spend| spend.transaction_id.chain == chain && spend.address == address).cloned().collect()
    }

    pub fn remove(&self, transaction_ids: &[TransactionId]) {
        self.spends().retain(|spend| !transaction_ids.contains(&spend.transaction_id));
    }

    fn spends(&self) -> MutexGuard<'_, Vec<PendingSpend>> {
        self.spends.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
