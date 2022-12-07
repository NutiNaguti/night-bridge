use near_sdk::serde_json::json;

const LITE_NODE_FILEPATH: &str = "../out/lite_node.wasm";

#[tokio::test]
async fn test_insert_filter() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let lite_node_wasm = std::fs::read(LITE_NODE_FILEPATH)?;
    let contract = worker.dev_deploy(&lite_node_wasm).await?;

    let encoded_logs = "MC8Rr8ETa1wI0MRqjDIRQQCtovKhBgEDjKEtAEqobARBIkQ0IiDiLImYDRg15EdRLhLpFBIR/giCEk5VEGT30BFgb7jcMaC8agUqWp14BWAE8ZahgtyyMQVSBwWYlQAOKgaUFNJEA0gCY1yf+EFpEDQIDFbAwecQYDIYdISKCsgQSgqmjKCGQ8BeKQXEEaGiCLGeGZI+QLgFAQBgM0yEVgKBi99QRiYcCaS8cKZKIAAqQHrBGNSGpBAwBNQxP2Y3D2DCktCQJZ8MSlqB3GQgVCSAQcwDsoASDh2V7imIcvRLnXkNugALYRASAgyAHRig4FgRmCo4cHAEQGUaAgVGYQ==";

    let query_args = json!({
        "request": {
            "block_number":1,
            "logs": encoded_logs.to_string()
        }
    });

    let outcome = contract
        .call("init")
        .args_json(json!({}))
        .max_gas()
        .transact()
        .await?;

    assert_eq!(true, outcome.is_success());

    let outcome = contract
        .call("insert_filter")
        .args_json(query_args)
        .max_gas()
        .transact()
        .await?;

    assert_eq!(true, outcome.is_success());

    let outcome = contract
        .call("view_filter")
        .args_json(json!({"block_number":1}))
        .max_gas()
        .transact()
        .await?;

    assert_eq!(true, outcome.is_success());

    let outcome_result: serde_json::Value = outcome.json()?;

    // let decoded_blooom = base64::decode(&outcome_result).unwrap();

    Ok(())
}
