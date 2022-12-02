use near_sdk::AccountId;
// macro allowing us to convert human readable units to workspace units.
// use near_units::parse_near;

use serde_json::{json, Value};

const BRIDGE_FILEPATH: &str = "../out/bridge.wasm";
const LITE_NODE_FILEPATH: &str = "../out/lite_node.wasm";

#[tokio::test]
async fn test_init() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(BRIDGE_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    let init_args = get_init_args_json();

    let outcome = contract
        .call("init")
        .args_json(init_args.clone())
        .max_gas()
        .transact()
        .await?;

    assert_eq!(true, outcome.is_success());

    let outcome: serde_json::Value = contract.call("view_state").view().await?.json()?;

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
    let bridge_contract = worker.dev_deploy(&bridge_wasm).await?;
    let lite_node_contract = worker.dev_deploy(&lite_node_wasm).await?;

    let init_args = get_init_args_json();

    let outcome = lite_node_contract
        .call("init")
        .args_json(json!({}))
        .max_gas()
        .transact()
        .await?;

    assert_eq!(true, outcome.is_success());

    let outcome = bridge_contract
        .call("init")
        .args_json(init_args.clone())
        .max_gas()
        .transact()
        .await?;

    assert_eq!(true, outcome.is_success());

    Ok(())
}

fn get_init_args_json() -> Value {
    let init_args = (
        "Locked(bytes32)".to_string(),
        "0x9431f9bba577B037D97ad6F7086a00eFB572c871".to_string(),
        "0x918DD8e3F443C1a8535d0F6F266EC20E3a9329e2".to_string(),
        "dev-1669803669965-75235193778699"
            .parse::<AccountId>()
            .unwrap(),
        "dev-1669804361266-30686725939679"
            .parse::<AccountId>()
            .unwrap(),
        vec!["nutinaguti.testnet".parse::<AccountId>().unwrap()],
    );

    json!({"eth_event_signature":init_args.0, "eth_bridge_address": init_args.1, "eth_token_address": init_args.2, "near_token_account": init_args.3, "lite_node_account": init_args.4, "admin_list": init_args.5})
}
