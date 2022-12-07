use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env::predecessor_account_id,
    near_bindgen, require,
    store::UnorderedMap,
    AccountId, BorshStorageKey, PanicOnDefault,
};

#[derive(BorshSerialize, BorshDeserialize, BorshStorageKey)]
pub enum StorageKey {
    Balance,
    Allowed,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct FunCoin {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub balance: UnorderedMap<AccountId, u64>,
    pub allowed: UnorderedMap<AccountId, UnorderedMap<AccountId, u64>>,
}

#[near_bindgen]
impl FunCoin {
    #[init]
    #[private]
    pub fn init(name: String, symbol: String, decimals: u8, total_supply: u64) -> Self {
        Self {
            name,
            symbol,
            decimals,
            total_supply,
            balance: UnorderedMap::new(StorageKey::Balance),
            allowed: UnorderedMap::new(StorageKey::Allowed),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn decimals(&self) -> &u8 {
        &self.decimals
    }

    pub fn total_supply(&self) -> &u64 {
        &self.total_supply
    }

    pub fn balance_of(&self, account_id: AccountId) -> &u64 {
        self.balance.get(&account_id).unwrap()
    }

    pub fn transfer(&mut self, to: AccountId, amount: u64) -> bool {
        let user_balance = self.balance_of(predecessor_account_id());
        require!(user_balance >= &amount);
        self.balance
            .insert(predecessor_account_id(), user_balance - amount)
            .unwrap();

        let receiver_balance = self.balance_of(to.clone());
        self.balance.insert(to, receiver_balance + amount).unwrap();

        true
    }

    pub fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: u64) -> bool {
        let user_balance = self.balance_of(from.clone());
        require!(user_balance >= &amount);
        require!(
            self.allowed
                .get(&from)
                .unwrap()
                .get(&predecessor_account_id())
                .unwrap()
                >= &amount
        );
        self.balance.insert(from, user_balance - amount).unwrap();

        let receiver_balance = self.balance_of(to.clone());
        self.balance.insert(to, receiver_balance + amount).unwrap();

        true
    }

    pub fn approve(&mut self, spender: AccountId, amount: u64) {
        let user_balance = self.balance_of(predecessor_account_id());
        require!(user_balance >= &amount);

        let allowance = self.allowed.get(&predecessor_account_id());
        if allowance.is_none() {
            // self.allowed.get(&predecessor_account_id()).unwrap().
        }
    }

    // pub fn allowance(&self) ->  {}
}
