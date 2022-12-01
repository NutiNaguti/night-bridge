use near_sdk::AccountId;
// macro allowing us to convert human readable units to workspace units.
// use near_units::parse_near;

use serde_json::json;

const WASM_FILEPATH: &str = "../out/bridge.wasm";

#[tokio::test]
async fn test_init() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    let outcome = contract
        .call("init")
        .args_json(json!(get_init_args()))
        .transact()
        .await?;

    assert_eq!(true, outcome.is_failure());

    let outcome = contract.view("view_state").await?;

    let contract_state = outcome.result;
    println!("{:?}", contract_state);
    // assert_eq!(get_init_args(), (contract_state.))

    Ok(())
}

fn get_init_args() -> (String, String, String, AccountId, AccountId, Vec<AccountId>) {
    (
        "Locked(bytes32)".to_string(),
        "0x9431f9bba577B037D97ad6F7086a00eFB572c871".to_string(),
        "0x918DD8e3F443C1a8535d0F6F266EC20E3a9329e2".to_string(),
        "dev-1669803669965-75235193778699".parse().unwrap(),
        "dev-1669804361266-30686725939679".parse().unwrap(),
        vec!["nutinaguti.testnet".parse().unwrap()],
    )
}
