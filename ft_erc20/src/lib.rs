use erc20::*;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env::predecessor_account_id,
    near_bindgen, require,
    store::{UnorderedSet, Vector},
    AccountId, BorshStorageKey, PanicOnDefault,
};

#[derive(BorshSerialize, BorshDeserialize, BorshStorageKey)]
pub enum StorageKey {
    AdminList,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct FunCoin {
    token: ERC20,
    admin_list: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl FunCoin {
    #[init]
    #[private]
    pub fn init(
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: u64,
        admin_list: Vec<String>,
    ) -> Self {
        let mut admins = UnorderedSet::new(StorageKey::AdminList);
        admins.extend(
            admin_list
                .iter()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<AccountId>>(),
        );
        Self {
            token: ERC20::init(name, symbol, decimals, total_supply),
            admin_list: admins,
        }
    }

    pub fn mint(&mut self, to: AccountId, value: u64) {
        require!(self.is_admin(predecessor_account_id()));
        self.token.mint(to, value);
    }

    pub fn burn(&mut self, account: AccountId, value: u64) {
        require!(self.is_admin(predecessor_account_id()));
        self.token.burn(account, value);
    }

    pub fn transfer(&mut self, to: AccountId, value: u64) {
        self.token.transfer(to, value);
    }

    pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: u64) {
        self.token.transfer_from(from, to, value);
    }

    pub fn approve(&mut self, spender: AccountId, value: u64) {
        self.token.approve(spender, value);
    }

    pub fn allowance(&self, owner: AccountId, spender: AccountId) -> &u64 {
        self.token.allowance(owner, spender)
    }

    fn is_admin(&self, caller: AccountId) -> bool {
        self.admin_list.contains(&caller)
    }
}
