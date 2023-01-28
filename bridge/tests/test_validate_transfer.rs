use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::U128,
    AccountId, ONE_NEAR,
};
use serde_json::{json, Value};

const BRIDGE_FILEPATH: &str = "../out/bridge.wasm";
const LITE_NODE_FILEPATH: &str = "../out/lite_node.wasm";
const ERC20_FILEPATH: &str = "../out/ft_erc20.wasm";
const ADMIN_ACCOUNT_ID: &str = "test.near";

#[derive(BorshSerialize, BorshDeserialize)]
pub struct InsertBloomFilterRequest {
    block_number: u64,
    logs: [u8; 256],
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ValidateTransferRequest {
    block_number: u64,
    receiver: AccountId,
    proof: String,
}

#[tokio::test]
async fn test_init() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let bridge_wasm = std::fs::read(BRIDGE_FILEPATH)?;
    let bridge_contract = worker.dev_deploy(&bridge_wasm).await?;

    let init_args = get_init_args_json(None, None);

    let outcome = bridge_contract
        .call("init")
        .args_json(init_args.clone())
        .max_gas()
        .transact()
        .await?;

    assert_eq!(true, outcome.is_success());

    let outcome: serde_json::Value = bridge_contract.call("view_state").view().await?.json()?;

    assert_eq!(*init_args.get("eth_event_signature").unwrap(), outcome[0]);
    assert_eq!(*init_args.get("eth_bridge_address").unwrap(), outcome[1]);
    assert_eq!(*init_args.get("eth_token_address").unwrap(), outcome[2]);
    assert_eq!(*init_args.get("near_token_account").unwrap(), outcome[3]);
    assert_eq!(*init_args.get("lite_node_account").unwrap(), outcome[4]);

    Ok(())
}

#[tokio::test]
async fn test_validate() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let bridge_wasm = std::fs::read(BRIDGE_FILEPATH)?;
    let lite_node_wasm = std::fs::read(LITE_NODE_FILEPATH)?;
    let erc20_wasm = std::fs::read(ERC20_FILEPATH)?;
    let bridge_contract = worker.dev_deploy(&bridge_wasm).await?;
    let lite_node_contract = worker.dev_deploy(&lite_node_wasm).await?;
    let erc20_contract = worker.dev_deploy(&erc20_wasm).await?;

    let outcome = erc20_contract
        .call("init")
        .args_json(json!({"name": "FunCoin", "symbol": "FUNC", "decimals": 18, "total_supply": U128(10u128.pow(18)), "admin_list": vec![ADMIN_ACCOUNT_ID]}))
        .max_gas()
        .transact()
        .await?;

    assert_eq!(true, outcome.is_success());

    let outcome = lite_node_contract
        .call("init")
        .args_json({})
        .max_gas()
        .transact()
        .await?;

    assert_eq!(true, outcome.is_success());

    let bloom_filter = InsertBloomFilterRequest {
        block_number: 8209810,
        logs: [
            48, 47, 17, 175, 193, 19, 107, 92, 8, 208, 196, 106, 140, 50, 17, 65, 0, 173, 162, 242,
            161, 6, 1, 3, 140, 161, 45, 0, 74, 168, 108, 4, 65, 34, 68, 52, 34, 32, 226, 44, 137,
            152, 13, 24, 53, 228, 71, 81, 46, 18, 233, 20, 18, 17, 254, 8, 130, 18, 78, 85, 16,
            100, 247, 208, 17, 96, 111, 184, 220, 49, 160, 188, 106, 5, 42, 90, 157, 120, 5, 96, 4,
            241, 150, 161, 130, 220, 178, 49, 5, 82, 7, 5, 152, 149, 0, 14, 42, 6, 148, 20, 210,
            68, 3, 72, 2, 99, 92, 159, 248, 65, 105, 16, 52, 8, 12, 86, 192, 193, 231, 16, 96, 50,
            24, 116, 132, 138, 10, 200, 16, 74, 10, 166, 140, 160, 134, 67, 192, 94, 41, 5, 196,
            17, 161, 162, 8, 177, 158, 25, 146, 62, 64, 184, 5, 1, 0, 96, 51, 76, 132, 86, 2, 129,
            139, 223, 80, 70, 38, 28, 9, 164, 188, 112, 166, 74, 32, 0, 42, 64, 122, 193, 24, 212,
            134, 164, 16, 48, 4, 212, 49, 63, 102, 55, 15, 96, 194, 146, 208, 144, 37, 159, 12, 74,
            90, 129, 220, 100, 32, 84, 36, 128, 65, 204, 3, 178, 128, 18, 14, 29, 149, 238, 41,
            136, 114, 244, 75, 157, 121, 13, 186, 0, 11, 97, 16, 18, 2, 12, 128, 29, 24, 160, 224,
            88, 17, 152, 42, 56, 112, 112, 4, 64, 101, 26, 2, 5, 70, 97,
        ],
    };

    let outcome = lite_node_contract
        .call("insert_filter")
        .args_borsh(bloom_filter)
        .max_gas()
        .transact()
        .await?;

    println!("{:#?}", outcome);

    assert_eq!(true, outcome.is_success());

    let init_args = get_init_args_json(Some(erc20_contract.id()), Some(lite_node_contract.id()));

    let outcome = bridge_contract
        .call("init")
        .args_json(init_args)
        .max_gas()
        .transact()
        .await?;

    assert_eq!(true, outcome.is_success());

    let validate_transfer_request = ValidateTransferRequest {
        block_number: 8209810u64,
        proof: "0xbd65fb7f0c8abd4d9aa3d3d1cc16ce83d3009f16a805682c2e7e524752a6adc8".to_owned(),
        receiver: ADMIN_ACCOUNT_ID.to_owned().parse().unwrap(),
    };
    let outcome = bridge_contract
        .call("validate_transfer")
        .args_borsh(validate_transfer_request)
        .max_gas()
        .deposit(ONE_NEAR)
        .transact()
        .await?;

    assert_eq!(true, outcome.is_success());

    Ok(())
}

fn get_init_args_json(near_token_account: Option<&str>, lite_node_account: Option<&str>) -> Value {
    let lite_node_account = lite_node_account.unwrap_or("dev-1669804361266-30686725939679");
    let near_token_account = near_token_account.unwrap_or("dev-1669803669965-75235193778699");
    json!({
        "eth_event_signature": "Locked(bytes32)",
        "eth_bridge_address": "0x9431f9bba577B037D97ad6F7086a00eFB572c871",
        "eth_token_address": "0x918DD8e3F443C1a8535d0F6F266EC20E3a9329e2",
        "near_token_account": near_token_account,
        "lite_node_account": lite_node_account,
        "admin_list": vec![ADMIN_ACCOUNT_ID]
    })
}
