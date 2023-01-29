use external::{fun_coin, line_node, TGAS};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::env;
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{self, collections::UnorderedSet, AccountId};
use near_sdk::{
    log, near_bindgen, require, BorshStorageKey, Gas, PanicOnDefault, Promise, PromiseError,
    ONE_NEAR, ONE_YOCTO,
};
use num::checked_pow;

mod external;

const VALIDATE_TRANSFER_FEE: u128 = ONE_NEAR;

#[derive(BorshSerialize, BorshDeserialize, BorshStorageKey)]
pub enum StorageKey {
    AdminSet,
    TransferMap,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ValidateTransferRequest {
    block_number: u64,
    receiver: AccountId,
    proof: String,
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

    pub fn view_state(
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
        #[serializer(borsh)] request: ValidateTransferRequest,
    ) -> Promise {
        if let Some(_) = self.validated_transfers.get(&request.proof) {
            panic!("Already completed transfer");
        }
        let deposit = env::attached_deposit();
        require!(deposit >= VALIDATE_TRANSFER_FEE, "Not enougth funds");
        let validate_request = external::ValidateRequest {
            block_number: request.block_number,
            eth_bridge_address: self.eth_bridge_address.clone(),
            event_signature: self.eth_event_signature.clone(),
            proof: request.proof.clone(),
        };
        let promise = line_node::ext(self.lite_node_account.clone())
            .with_static_gas(Gas(10 * TGAS))
            .validate(validate_request);

        promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(Gas(10 * TGAS))
                .with_attached_deposit(ONE_YOCTO)
                .validate_callback(request),
        )
    }

    #[private]
    #[payable]
    pub fn validate_callback(
        &mut self,
        #[serializer(borsh)] request: ValidateTransferRequest,
        #[callback_result] call_result: Result<bool, PromiseError>,
    ) -> Promise {
        if call_result.is_err() {
            panic!("There was an error contacting Lite Node");
        }

        let call_result = call_result.unwrap();
        log!("call_result: {}", call_result);

        if call_result {
            let amount = U128(100 * (checked_pow(10, 18).unwrap()));
            log!("amount: {:?}", amount);
            let promise = fun_coin::ext(self.near_token_account.clone())
                .with_static_gas(Gas(5 * TGAS))
                .mint(request.receiver.clone(), amount);

            promise.then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(10 * TGAS))
                    .complete_transfer_callback(request),
            )
        } else {
            panic!("Invalid proof");
        }
    }

    #[private]
    pub fn complete_transfer_callback(
        &mut self,
        #[serializer(borsh)] request: ValidateTransferRequest,
        #[callback_result] call_result: Result<(), PromiseError>,
    ) {
        if call_result.is_err() {
            panic!("{:?}", call_result);
        } else {
            //TODO
            log!("Tokens transfered");
            self.validated_transfers.insert(
                &request.proof,
                &Transfer {
                    near_timestamp: env::block_timestamp(),
                    eth_block_number: request.block_number,
                    receiver: request.receiver,
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

    pub fn withdrow_fees(&self, _admin_index: usize) {
        require!(self.admin_set.contains(&env::predecessor_account_id()));
        //TODO
    }

    pub fn set_eth_event_signature(&mut self, eth_event_signature: String) {
        require!(self.admin_set.contains(&env::predecessor_account_id()));
        self.eth_event_signature = eth_event_signature;
    }

    pub fn set_eth_bridge_address(&mut self, eth_bridge_address: String) {
        require!(self.admin_set.contains(&env::predecessor_account_id()));
        self.eth_bridge_address = eth_bridge_address;
    }

    pub fn set_eth_token_address(&mut self, eth_token_address: String) {
        require!(self.admin_set.contains(&env::predecessor_account_id()));
        self.eth_token_address = eth_token_address;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{test_utils::VMContextBuilder, testing_env};

    const ETH_EVENT_SIGNATURE: &str = "Locked(bytes32)";
    const ETH_BRIDGE_ADDRESS: &str = "0x9431f9bba577B037D97ad6F7086a00eFB572c871";
    const ETH_TOKEN_ADDRESS: &str = "0x918DD8e3F443C1a8535d0F6F266EC20E3a9329e2";
    const NEAR_TOKEN_ACCOUNT: &str = "dev-1669803669965-75235193778699";
    const LITE_NODE_ACCOUNT: &str = "dev-1669804361266-30686725939679";
    const ADMIN: &str = "nutinaguti.testnet";

    fn get_context(account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(account_id);
        builder
    }

    #[test]
    fn test_init() {
        let predecessor_account_id = "nutinaguti.testnet".parse().unwrap();
        let context = get_context(predecessor_account_id);
        testing_env!(context.build());

        let admin_list = vec![ADMIN.parse().unwrap()];

        let contract = Bridge::init(
            ETH_EVENT_SIGNATURE.to_string(),
            ETH_BRIDGE_ADDRESS.to_string(),
            ETH_TOKEN_ADDRESS.to_string(),
            NEAR_TOKEN_ACCOUNT.parse().unwrap(),
            LITE_NODE_ACCOUNT.parse().unwrap(),
            admin_list.clone(),
        );

        let state = contract.view_state();
        assert_eq!(
            state,
            (
                ETH_EVENT_SIGNATURE.to_string(),
                ETH_BRIDGE_ADDRESS.to_string(),
                ETH_TOKEN_ADDRESS.to_string(),
                NEAR_TOKEN_ACCOUNT.parse().unwrap(),
                LITE_NODE_ACCOUNT.parse().unwrap(),
                admin_list,
                0
            )
        );
    }
}
