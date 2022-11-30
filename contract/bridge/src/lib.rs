use external::{fun_coin, line_node, TGAS};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::env;
use near_sdk::serde::Serialize;
use near_sdk::{self, collections::UnorderedSet, AccountId};
use near_sdk::{
    json_types, log, near_bindgen, require, BorshStorageKey, Gas, PanicOnDefault, Promise,
    PromiseError, ONE_NEAR, ONE_YOCTO,
};

mod external;

const VALIDATE_TRANSFER_FEE: u128 = ONE_NEAR;

#[derive(BorshSerialize, BorshDeserialize, BorshStorageKey)]
pub enum StorageKey {
    AdminSet,
    TransferMap,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Transfer {
    near_timestamp: u64,
    eth_block_number: u64,
    receiver: AccountId,
}

#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Bridge {
    eth_event_signature: String,
    eth_bridge_address: String,
    eth_token_address: String,
    near_token_account: AccountId,
    lite_node_account: AccountId,
    admin_set: UnorderedSet<AccountId>,
    validated_transfers: UnorderedMap<String, Transfer>,
}

#[near_bindgen]
impl Bridge {
    #[init]
    #[private]
    pub fn init(
        eth_event_signature: String,
        eth_bridge_address: String,
        eth_token_address: String,
        near_token_account: AccountId,
        lite_node_account: AccountId,
        admin_list: Vec<AccountId>,
    ) -> Self {
        let mut admin_set = UnorderedSet::new(StorageKey::AdminSet);
        for e in admin_list.iter() {
            admin_set.insert(e);
        }

        fun_coin::ext(near_token_account.clone())
            .with_static_gas(Gas(5 * TGAS))
            .internal_register(env::current_account_id());

        Self {
            eth_event_signature,
            eth_bridge_address,
            eth_token_address,
            near_token_account,
            lite_node_account,
            admin_set,
            validated_transfers: UnorderedMap::new(StorageKey::TransferMap),
        }
    }

    pub fn view_storage(
        &self,
    ) -> (
        String,
        String,
        String,
        AccountId,
        AccountId,
        Vec<AccountId>,
        u64,
    ) {
        (
            self.eth_event_signature.clone(),
            self.eth_bridge_address.clone(),
            self.eth_token_address.clone(),
            self.near_token_account.clone(),
            self.lite_node_account.clone(),
            self.admin_set.to_vec(),
            self.validated_transfers.len(),
        )
    }

    // =========== From ETH transfer functions ===========
    // ---------------------------------------------------

    pub fn view_validated_transfer(&self, proof: String) -> Option<Transfer> {
        self.validated_transfers.get(&proof)
    }

    #[payable]
    pub fn validate_transfer(
        &mut self,
        block_number: u64,
        receiver: AccountId,
        proof: String,
    ) -> Promise {
        if let Some(_) = self.validated_transfers.get(&proof) {
            panic!("Already completed transfer");
        }
        let deposit = env::attached_deposit();
        require!(deposit >= VALIDATE_TRANSFER_FEE, "Not enougth funds");
        let promise = line_node::ext(self.lite_node_account.clone())
            .with_static_gas(Gas(10 * TGAS))
            .validate(
                block_number,
                self.eth_bridge_address.clone(),
                self.eth_event_signature.clone(),
                proof.clone(),
            );

        promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(Gas(10 * TGAS))
                .with_attached_deposit(ONE_YOCTO)
                .validate_callback(block_number, receiver, &proof),
        )
    }

    #[private]
    #[payable]
    pub fn validate_callback(
        &mut self,
        block_number: u64,
        receiver: AccountId,
        proof: &String,
        #[callback_result] call_result: Result<bool, PromiseError>,
    ) -> Promise {
        if call_result.is_err() {
            panic!("There was an error contacting Lite Node");
        }

        let call_result = call_result.unwrap();
        log!("call_result: {}", call_result);

        if call_result {
            // TODO if user doesn't registered may need to register him
            let amount = 1;
            let deposit = env::attached_deposit();
            let promise = fun_coin::ext(self.near_token_account.clone())
                .with_static_gas(Gas(5 * TGAS))
                .mint(receiver.clone(), json_types::U128(amount));

            promise.then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(10 * TGAS))
                    .complete_transfer_callback(block_number, receiver, proof),
            )
        } else {
            panic!("Proof are invalid");
        }
    }

    #[private]
    pub fn complete_transfer_callback(
        &mut self,
        #[callback_result] call_result: Result<(), PromiseError>,
        block_number: u64,
        receiver: AccountId,
        proof: &String,
    ) {
        if call_result.is_err() {
            panic!("{:?}", call_result);
        } else {
            //TODO
            self.validated_transfers.insert(
                &proof,
                &Transfer {
                    near_timestamp: env::block_timestamp(),
                    eth_block_number: block_number,
                    receiver,
                },
            );
        }
    }

    // =========== To ETH transfer functions ===========
    // -------------------------------------------------

    #[payable]
    pub fn lock(&mut self) {
        todo!()
    }

    // =========== Admin functions ===========
    // ---------------------------------------

    pub fn remove_validated_transfer(&mut self, proof: String) {
        require!(self.admin_set.contains(&env::predecessor_account_id()));
        self.validated_transfers.remove(&proof);
    }

    pub fn set_near_token(&mut self, near_token_account: AccountId) {
        require!(self.admin_set.contains(&env::predecessor_account_id()));
        self.near_token_account = near_token_account;
    }

    pub fn set_lite_node(&mut self, lite_node_account: AccountId) {
        require!(self.admin_set.contains(&env::predecessor_account_id()));
        self.lite_node_account = lite_node_account;
    }

    pub fn add_new_admin(&mut self, admin_account: AccountId) {
        require!(self.admin_set.contains(&env::predecessor_account_id()));
        match self.admin_set.contains(&admin_account) {
            true => panic!("This account already in adim_set"),
            false => self.admin_set.insert(&admin_account),
        };
    }

    pub fn withdrow_fees(&self, admin_index: usize) {
        require!(self.admin_set.contains(&env::predecessor_account_id()));
        //TODO
    }
}
