use near_sdk::{
    self,
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedSet,
    require,
    store::UnorderedMap,
    PanicOnDefault,
};
use near_sdk::{near_bindgen, BorshStorageKey};

pub type TransactionId = [u8; 32];

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    EventLogs,
    Topics,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct EventLogs {
    topics: UnorderedSet<Vec<u8>>,
    block_hash: Vec<u8>,
}

#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct LiteClient {
    event_logs: UnorderedMap<TransactionId, EventLogs>,
}

#[near_bindgen]
impl LiteClient {
    #[init]
    #[private]
    pub fn init() -> Self {
        Self {
            event_logs: UnorderedMap::new(StorageKey::EventLogs),
        }
    }

    pub fn get_event_logs(&self, transaction_id: [u8; 32]) -> &EventLogs {
        let transaction_id = transaction_id as TransactionId;
        match self.event_logs.get(&transaction_id) {
            Some(logs) => return logs,
            _ => {
                panic!("Logs doesn't exist")
            }
        }
    }

    pub fn add_event_logs(
        &mut self,
        transaction_id: [u8; 32],
        topics: Vec<Vec<u8>>,
        block_hash: Vec<u8>,
    ) {
        let mut logs = EventLogs {
            topics: UnorderedSet::new(StorageKey::Topics),
            block_hash,
        };

        for e in topics.iter() {
            logs.topics.insert(e);
        }

        self.event_logs.insert(transaction_id, logs);
    }
}

fn if_caller_is_admin() -> bool {
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;

    fn get_context() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
    }
}
