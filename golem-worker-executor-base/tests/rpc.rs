use crate::common;
use crate::common::{val_float32, val_list, val_record, val_string, val_u64};
use assert2::check;
use golem_wasm_rpc::protobuf::Val;
use std::collections::HashMap;
use std::path::Path;
use std::time::SystemTime;

#[tokio::test]
async fn auction_example_1() {
    let context = common::TestContext::new();
    let mut executor = common::start(&context).await.unwrap();

    let registry_template_id = executor.store_template(Path::new(
        "../test-templates/auction_registry_composed.wasm",
    ));
    let auction_template_id = executor.store_template(Path::new("../test-templates/auction.wasm"));

    let mut env = HashMap::new();
    env.insert(
        "AUCTION_TEMPLATE_ID".to_string(),
        auction_template_id.to_string(),
    );
    let registry_worker_id = executor
        .try_start_worker_versioned(&registry_template_id, 0, "auction-registry-1", vec![], env)
        .await
        .unwrap();

    let _ = executor.log_output(&registry_worker_id).await;

    let expiration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let create_auction_result = executor
        .invoke_and_await(
            &registry_worker_id,
            "auction:registry/api/create-auction",
            vec![
                val_string("test-auction"),
                val_string("this is a test"),
                val_float32(100.0),
                val_u64(expiration + 600),
            ],
        )
        .await;

    let get_auctions_result = executor
        .invoke_and_await(
            &registry_worker_id,
            "auction:registry/api/get-auctions",
            vec![],
        )
        .await;

    drop(executor);

    println!("result: {:?}", create_auction_result);
    println!("result: {:?}", get_auctions_result);
    check!(create_auction_result.is_ok());

    let Val {
        val: Some(auction_id),
    } = &create_auction_result.unwrap()[0]
    else {
        panic!("expected result");
    };

    check!(
        get_auctions_result
            == Ok(vec![val_list(vec![val_record(vec![
                Val {
                    val: Some(auction_id.clone())
                },
                val_string("test-auction"),
                val_string("this is a test"),
                val_float32(100.0),
                val_u64(expiration + 600)
            ]),])])
    );
}
