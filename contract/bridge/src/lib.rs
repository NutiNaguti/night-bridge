use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::predecessor_account_id;
use near_sdk::{self, collections::UnorderedSet, AccountId};
use near_sdk::{near_bindgen, BorshStorageKey, PanicOnDefault};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Request {
    amount: u128,
    user: AccountId,
}

#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Bridge {
    queu: UnorderedSet<Request>,
    admin_list: UnorderedSet<AccountId>,
}

#[derive(BorshSerialize, BorshDeserialize, BorshStorageKey)]
pub enum StorageKey {
    Queu,
    AdminList,
}

#[near_bindgen]
impl Bridge {
    #[init]
    #[private]
    pub fn init() -> Self {
        Self {
            queu: UnorderedSet::new(StorageKey::Queu),
            admin_list: UnorderedSet::new(StorageKey::AdminList),
        }
    }

    pub fn check_admin(&self) -> bool {
        self.admin_list.contains(&predecessor_account_id())
    }

    pub fn withdrow_tokens(&mut self) {
        unimplemented!()
    }

    pub fn send_tokens(&mut self, address: String, amount: u128) {
        unimplemented!()
    }
}
