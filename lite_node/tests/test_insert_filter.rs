use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde_json::json,
};

const LITE_NODE_FILEPATH: &str = "../out/lite_node.wasm";

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct InsertBloomFilterRequest {
    block_number: u64,
    logs: [u8; 256],
}

#[tokio::test]
async fn test_insert_filter() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let lite_node_wasm = std::fs::read(LITE_NODE_FILEPATH)?;
    let contract = worker.dev_deploy(&lite_node_wasm).await?;

    let outcome = contract
        .call("init")
        .args_json(json!({}))
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

    let outcome = contract
        .call("insert_filter")
        .args_borsh(bloom_filter)
        .max_gas()
        .transact()
        .await?;

    println!("{:#?}", outcome);
    assert_eq!(true, outcome.is_success());

    let outcome = contract
        .call("view_filter")
        .args_json(json!({"block_number":8209810}))
        .max_gas()
        .transact()
        .await?;

    println!("{:#?}", outcome);
    assert_eq!(true, outcome.is_success());

    // let _outcome_result: serde_json::Value = outcome.json()?;

    // let decoded_blooom = base64::decode(&outcome_result).unwrap();

    Ok(())
}
