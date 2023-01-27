use near_sdk::{json_types::U128, ONE_NEAR};
use serde_json::{json, Value};

const BRIDGE_FILEPATH: &str = "../out/bridge.wasm";
const LITE_NODE_FILEPATH: &str = "../out/lite_node.wasm";
const ERC20_FILEPATH: &str = "../out/ft_erc20.wasm";
const ADMIN_ACCOUNT_ID: &str = "test.near";

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

    let bloom_filter = get_mock_bloom_filter();
    let outcome = lite_node_contract
        .call("insert_filter")
        .args_json(bloom_filter)
        .max_gas()
        .transact()
        .await?;

    assert_eq!(true, outcome.is_success());

    let init_args = get_init_args_json(Some(erc20_contract.id()), Some(lite_node_contract.id()));

    let outcome = bridge_contract
        .call("init")
        .args_json(init_args)
        .max_gas()
        .transact()
        .await?;

    assert_eq!(true, outcome.is_success());

    let mock_transfer_data = get_mock_transfer_data();
    let receiver = ADMIN_ACCOUNT_ID;
    let outcome = bridge_contract
        .call("validate_transfer")
        .args_json(json!({"block_number": mock_transfer_data.0, "receiver": receiver, "proof": mock_transfer_data.1}))
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

fn get_mock_transfer_data() -> (u64, String) {
    let block_number = 8209810u64;
    let proof = "0xbd65fb7f0c8abd4d9aa3d3d1cc16ce83d3009f16a805682c2e7e524752a6adc8";
    (block_number, proof.to_string())
}

fn get_mock_bloom_filter() -> Value {
    json!({"request": {
      "block_number": 8209810,
      "logs": "4yQZBhm08HQKAQk9gRgUASAJ6mIeVCgyKEkJUE6KRTRABGEBAgEa1FMikULEBACRyMIFsAIzueCdAGKOjC9B0QhhUBLEcyqoOlAGLEgMhGQSEIZlQ/SxxCEVIKjDgCggf2USctbmSKAqSRKQHIibVW0AxlRY2nIUMCI4lISYL5V1EAWGCADCICHGGBQCGAESoSoFxSoIQF+wEmDxECIQTgYJyMJUICpS0iAK8Od7KCSmRMCgCIxKACgwgACUsAxAySDRqxaADQIWsCFoKHLSUQmhhTMorIETUNOYE4WS4iCft6kKB+CGEAowot5qURGmIbgQ2EjwREEAzDAQqAFQAQ=="
    }})
}
