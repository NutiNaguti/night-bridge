use bloom::Bloom;
use near_sdk::{
    self, base64,
    borsh::{self, BorshDeserialize, BorshSerialize},
    env::{keccak256, log},
    log,
    store::UnorderedMap,
    PanicOnDefault,
};
use near_sdk::{near_bindgen, BorshStorageKey};

mod bloom;
mod utils;

type BlockNumber = u64;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ValidateRequest {
    block_number: u64,
    eth_bridge_address: String,
    event_signature: String,
    proof: String,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct InsertBloomFilterRequest {
    block_number: u64,
    logs: [u8; 256],
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    LogsFilter,
}

#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct LiteNode {
    logs_filter: UnorderedMap<BlockNumber, Bloom>,
}

#[near_bindgen]
impl LiteNode {
    #[init]
    #[private]
    pub fn init() -> Self {
        Self {
            logs_filter: UnorderedMap::new(StorageKey::LogsFilter),
        }
    }

    #[payable]
    pub fn validate(&mut self, #[serializer(borsh)] request: ValidateRequest) -> bool {
        let bloom = self.logs_filter.get(&request.block_number).unwrap();
        let eth_bridge_address = utils::decode_hex(&request.eth_bridge_address).unwrap();
        let proof = utils::decode_hex(&request.proof).unwrap();

        let event_signature: [u8; 32] = keccak256(&keccak256(request.event_signature.as_bytes()))
            .try_into()
            .unwrap();

        let contract_address: [u8; 32] = keccak256(&eth_bridge_address).try_into().unwrap();

        let proof: [u8; 32] = keccak256(&proof).try_into().unwrap();

        bloom.contains_input(event_signature)
            & bloom.contains_input(contract_address)
            & bloom.contains_input(proof)
    }

    #[result_serializer(borsh)]
    pub fn view_filter(&self, block_number: BlockNumber) -> Option<&Bloom> {
        self.logs_filter.get(&block_number)
    }

    pub fn insert_filter(&mut self, #[serializer(borsh)] request: InsertBloomFilterRequest) {
        log!("{:#?}", request);
        match self.logs_filter.get(&request.block_number) {
            Some(_) => panic!("Already added"),
            _ => self
                .logs_filter
                .insert(request.block_number, Bloom { logs: request.logs }),
        };
    }

    pub fn remove_filter(&mut self, block_number: BlockNumber) {
        self.logs_filter.remove(&block_number);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{test_utils::VMContextBuilder, testing_env};

    const LOGS: [u8; 256] = [
        48, 47, 17, 175, 193, 19, 107, 92, 8, 208, 196, 106, 140, 50, 17, 65, 0, 173, 162, 242,
        161, 6, 1, 3, 140, 161, 45, 0, 74, 168, 108, 4, 65, 34, 68, 52, 34, 32, 226, 44, 137, 152,
        13, 24, 53, 228, 71, 81, 46, 18, 233, 20, 18, 17, 254, 8, 130, 18, 78, 85, 16, 100, 247,
        208, 17, 96, 111, 184, 220, 49, 160, 188, 106, 5, 42, 90, 157, 120, 5, 96, 4, 241, 150,
        161, 130, 220, 178, 49, 5, 82, 7, 5, 152, 149, 0, 14, 42, 6, 148, 20, 210, 68, 3, 72, 2,
        99, 92, 159, 248, 65, 105, 16, 52, 8, 12, 86, 192, 193, 231, 16, 96, 50, 24, 116, 132, 138,
        10, 200, 16, 74, 10, 166, 140, 160, 134, 67, 192, 94, 41, 5, 196, 17, 161, 162, 8, 177,
        158, 25, 146, 62, 64, 184, 5, 1, 0, 96, 51, 76, 132, 86, 2, 129, 139, 223, 80, 70, 38, 28,
        9, 164, 188, 112, 166, 74, 32, 0, 42, 64, 122, 193, 24, 212, 134, 164, 16, 48, 4, 212, 49,
        63, 102, 55, 15, 96, 194, 146, 208, 144, 37, 159, 12, 74, 90, 129, 220, 100, 32, 84, 36,
        128, 65, 204, 3, 178, 128, 18, 14, 29, 149, 238, 41, 136, 114, 244, 75, 157, 121, 13, 186,
        0, 11, 97, 16, 18, 2, 12, 128, 29, 24, 160, 224, 88, 17, 152, 42, 56, 112, 112, 4, 64, 101,
        26, 2, 5, 70, 97,
    ];

    const PROOF: &str = "0x3b874d464775b5082b95c98fb5f815494cc129e32c4e8a07a0bb98e710f8c25c";
    const EVENT_SIGNATURE: &str = "Locked(bytes32)";
    const ETH_CONTRACT_ADDRESS: &str = "0x9431f9bba577B037D97ad6F7086a00eFB572c871";

    fn get_context(predecessor: String) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor.parse().unwrap());
        builder
    }

    #[test]
    fn test_insert_filter() {
        let predecessor = "foo".to_string();
        let context = get_context(predecessor);
        testing_env!(context.build());

        let mut contract = LiteNode::init();

        let request = InsertBloomFilterRequest {
            block_number: 1,
            logs: LOGS,
        };

        contract.insert_filter(request);
        let logs_filter = contract.logs_filter.get(&1u64).unwrap();
        assert_eq!(LOGS, logs_filter.logs);
    }

    #[test]
    fn test_validate() {
        let predecessor = "foo".to_string();
        let mut context = get_context(predecessor);
        context.attached_deposit(1);
        testing_env!(context.build());

        let mut contract = LiteNode::init();

        let request = InsertBloomFilterRequest {
            block_number: 1,
            logs: LOGS,
        };

        contract.insert_filter(request);
        let validate_request = ValidateRequest {
            block_number: 1,
            eth_bridge_address: ETH_CONTRACT_ADDRESS.to_string(),
            event_signature: EVENT_SIGNATURE.to_string(),
            proof: PROOF.to_string(),
        };
        let validated = contract.validate(validate_request);
        assert_eq!(true, validated);
    }
}
