use crate::common::{start, TestContext};
use assert2::check;
use golem_test_framework::dsl::TestDsl;
use golem_wasm_rpc::Value;

#[tokio::test]
#[tracing::instrument]
async fn readwrite_get_returns_the_value_that_was_set() {
    let context = TestContext::new();
    let executor = start(&context).await.unwrap();

    let component_id = executor.store_component("key-value-service").await;
    let worker_name = "key-value-service-1";
    let worker_id = executor.start_worker(&component_id, worker_name).await;

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key".to_string()),
                Value::List(vec![Value::U8(1), Value::U8(2), Value::U8(3)]),
            ],
        )
        .await
        .unwrap();

    let result = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/get",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key".to_string()),
            ],
        )
        .await
        .unwrap();

    drop(executor);

    check!(
        result
            == vec![Value::Option(Some(Box::new(Value::List(vec![
                Value::U8(1),
                Value::U8(2),
                Value::U8(3),
            ]))))]
    );
}

#[tokio::test]
#[tracing::instrument]
async fn readwrite_get_fails_if_the_value_was_not_set() {
    let context = TestContext::new();
    let executor = start(&context).await.unwrap();

    let component_id = executor.store_component("key-value-service").await;
    let worker_name = "key-value-service-2";
    let worker_id = executor.start_worker(&component_id, worker_name).await;

    let result = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/get",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key".to_string()),
            ],
        )
        .await
        .unwrap();

    drop(executor);

    check!(result == vec![Value::Option(None)]);
}

#[tokio::test]
#[tracing::instrument]
async fn readwrite_set_replaces_the_value_if_it_was_already_set() {
    let context = TestContext::new();
    let executor = start(&context).await.unwrap();

    let component_id = executor.store_component("key-value-service").await;
    let worker_name = "key-value-service-3";
    let worker_id = executor.start_worker(&component_id, worker_name).await;

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key".to_string()),
                Value::List(vec![Value::U8(1), Value::U8(2), Value::U8(3)]),
            ],
        )
        .await
        .unwrap();

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key".to_string()),
                Value::List(vec![Value::U8(4), Value::U8(5), Value::U8(6)]),
            ],
        )
        .await
        .unwrap();

    let result = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/get",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key".to_string()),
            ],
        )
        .await
        .unwrap();

    drop(executor);

    check!(
        result
            == vec![Value::Option(Some(Box::new(Value::List(vec![
                Value::U8(4),
                Value::U8(5),
                Value::U8(6),
            ]))))]
    );
}

#[tokio::test]
#[tracing::instrument]
async fn readwrite_delete_removes_the_value_if_it_was_already_set() {
    let context = TestContext::new();
    let executor = start(&context).await.unwrap();

    let component_id = executor.store_component("key-value-service").await;
    let worker_name = "key-value-service-4";
    let worker_id = executor.start_worker(&component_id, worker_name).await;

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key".to_string()),
                Value::List(vec![Value::U8(1), Value::U8(2), Value::U8(3)]),
            ],
        )
        .await
        .unwrap();

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/delete",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key".to_string()),
            ],
        )
        .await
        .unwrap();

    let result = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/get",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key".to_string()),
            ],
        )
        .await
        .unwrap();

    drop(executor);

    check!(result == vec![Value::Option(None)]);
}

#[tokio::test]
#[tracing::instrument]
async fn readwrite_exists_returns_true_if_the_value_was_set() {
    let context = TestContext::new();
    let executor = start(&context).await.unwrap();

    let component_id = executor.store_component("key-value-service").await;
    let worker_name = "key-value-service-5";
    let worker_id = executor.start_worker(&component_id, worker_name).await;

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key".to_string()),
                Value::List(vec![Value::U8(1), Value::U8(2), Value::U8(3)]),
            ],
        )
        .await
        .unwrap();

    let result = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/exists",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key".to_string()),
            ],
        )
        .await
        .unwrap();

    drop(executor);

    check!(result == vec![Value::Bool(true)]);
}

#[tokio::test]
#[tracing::instrument]
async fn readwrite_exists_returns_false_if_the_value_was_not_set() {
    let context = TestContext::new();
    let executor = start(&context).await.unwrap();

    let component_id = executor.store_component("key-value-service").await;
    let worker_name = "key-value-service-6";
    let worker_id = executor.start_worker(&component_id, worker_name).await;

    let result = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/exists",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key".to_string()),
            ],
        )
        .await
        .unwrap();

    drop(executor);

    check!(result == vec![Value::Bool(false)]);
}

#[tokio::test]
#[tracing::instrument]
async fn readwrite_buckets_can_be_shared_between_workers() {
    let context = TestContext::new();
    let executor = start(&context).await.unwrap();

    let component_id = executor.store_component("key-value-service").await;
    let worker_id_1 = executor
        .start_worker(&component_id, "key-value-service-7")
        .await;
    let worker_id_2 = executor
        .start_worker(&component_id, "key-value-service-8")
        .await;

    let _ = executor
        .invoke_and_await(
            &worker_id_1,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-bucket")),
                Value::String("key".to_string()),
                Value::List(vec![Value::U8(1), Value::U8(2), Value::U8(3)]),
            ],
        )
        .await
        .unwrap();

    let result = executor
        .invoke_and_await(
            &worker_id_2,
            "golem:it/api/get",
            vec![
                Value::String(format!("{component_id}-bucket")),
                Value::String("key".to_string()),
            ],
        )
        .await
        .unwrap();

    drop(executor);

    check!(
        result
            == vec![Value::Option(Some(Box::new(Value::List(vec![
                Value::U8(1),
                Value::U8(2),
                Value::U8(3),
            ]))))]
    );
}

#[tokio::test]
#[tracing::instrument]
async fn batch_get_many_gets_multiple_values() {
    let context = TestContext::new();
    let executor = start(&context).await.unwrap();

    let component_id = executor.store_component("key-value-service").await;
    let worker_name = "key-value-service-9";
    let worker_id = executor.start_worker(&component_id, worker_name).await;

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key1".to_string()),
                Value::List(vec![Value::U8(1), Value::U8(2), Value::U8(3)]),
            ],
        )
        .await
        .unwrap();

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key2".to_string()),
                Value::List(vec![Value::U8(4), Value::U8(5), Value::U8(6)]),
            ],
        )
        .await
        .unwrap();

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key3".to_string()),
                Value::List(vec![Value::U8(7), Value::U8(8), Value::U8(9)]),
            ],
        )
        .await
        .unwrap();

    let result = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/get-many",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::List(vec![
                    Value::String("key1".to_string()),
                    Value::String("key2".to_string()),
                    Value::String("key3".to_string()),
                ]),
            ],
        )
        .await
        .unwrap();

    drop(executor);

    check!(
        result
            == vec![Value::Option(Some(Box::new(Value::List(vec![
                Value::List(vec![Value::U8(1), Value::U8(2), Value::U8(3),]),
                Value::List(vec![Value::U8(4), Value::U8(5), Value::U8(6),]),
                Value::List(vec![Value::U8(7), Value::U8(8), Value::U8(9),])
            ]))))]
    );
}

#[tokio::test]
#[tracing::instrument]
async fn batch_get_many_fails_if_any_value_was_not_set() {
    let context = TestContext::new();
    let executor = start(&context).await.unwrap();

    let component_id = executor.store_component("key-value-service").await;
    let worker_name = "key-value-service-10";
    let worker_id = executor.start_worker(&component_id, worker_name).await;

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key1".to_string()),
                Value::List(vec![Value::U8(1), Value::U8(2), Value::U8(3)]),
            ],
        )
        .await
        .unwrap();

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key2".to_string()),
                Value::List(vec![Value::U8(4), Value::U8(5), Value::U8(6)]),
            ],
        )
        .await
        .unwrap();

    let result = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/get-many",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::List(vec![
                    Value::String("key1".to_string()),
                    Value::String("key2".to_string()),
                    Value::String("key3".to_string()),
                ]),
            ],
        )
        .await
        .unwrap();

    drop(executor);

    check!(result == vec![Value::Option(None)]);
}

#[tokio::test]
#[tracing::instrument]
async fn batch_set_many_sets_multiple_values() {
    let context = TestContext::new();
    let executor = start(&context).await.unwrap();

    let component_id = executor.store_component("key-value-service").await;
    let worker_name = "key-value-service-11";
    let worker_id = executor.start_worker(&component_id, worker_name).await;

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set-many",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::List(vec![
                    Value::Tuple(vec![
                        Value::String("key1".to_string()),
                        Value::List(vec![Value::U8(1), Value::U8(2), Value::U8(3)]),
                    ]),
                    Value::Tuple(vec![
                        Value::String("key2".to_string()),
                        Value::List(vec![Value::U8(4), Value::U8(5), Value::U8(6)]),
                    ]),
                    Value::Tuple(vec![
                        Value::String("key3".to_string()),
                        Value::List(vec![Value::U8(7), Value::U8(8), Value::U8(9)]),
                    ]),
                ]),
            ],
        )
        .await
        .unwrap();

    let result1 = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/get",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key1".to_string()),
            ],
        )
        .await
        .unwrap();

    let result2 = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/get",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key2".to_string()),
            ],
        )
        .await
        .unwrap();

    let result3 = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/get",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key3".to_string()),
            ],
        )
        .await
        .unwrap();

    drop(executor);

    check!(
        result1
            == vec![Value::Option(Some(Box::new(Value::List(vec![
                Value::U8(1),
                Value::U8(2),
                Value::U8(3),
            ]))))]
    );
    check!(
        result2
            == vec![Value::Option(Some(Box::new(Value::List(vec![
                Value::U8(4),
                Value::U8(5),
                Value::U8(6),
            ]))))]
    );
    check!(
        result3
            == vec![Value::Option(Some(Box::new(Value::List(vec![
                Value::U8(7),
                Value::U8(8),
                Value::U8(9),
            ]))))]
    );
}

#[tokio::test]
#[tracing::instrument]
async fn batch_delete_many_deletes_multiple_values() {
    let context = TestContext::new();
    let executor = start(&context).await.unwrap();

    let component_id = executor.store_component("key-value-service").await;
    let worker_name = "key-value-service-12";
    let worker_id = executor.start_worker(&component_id, worker_name).await;

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key1".to_string()),
                Value::List(vec![Value::U8(1), Value::U8(2), Value::U8(3)]),
            ],
        )
        .await
        .unwrap();

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key2".to_string()),
                Value::List(vec![Value::U8(4), Value::U8(5), Value::U8(6)]),
            ],
        )
        .await
        .unwrap();

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key3".to_string()),
                Value::List(vec![Value::U8(7), Value::U8(8), Value::U8(9)]),
            ],
        )
        .await
        .unwrap();

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/delete-many",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::List(vec![
                    Value::String("key1".to_string()),
                    Value::String("key2".to_string()),
                    Value::String("key3".to_string()),
                ]),
            ],
        )
        .await
        .unwrap();

    let result1 = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/get",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key1".to_string()),
            ],
        )
        .await
        .unwrap();

    let result2 = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/get",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key2".to_string()),
            ],
        )
        .await
        .unwrap();

    let result3 = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/get",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key3".to_string()),
            ],
        )
        .await
        .unwrap();

    drop(executor);

    check!(result1 == vec![Value::Option(None)]);
    check!(result2 == vec![Value::Option(None)]);
    check!(result3 == vec![Value::Option(None)]);
}

#[tokio::test]
#[tracing::instrument]
async fn batch_get_keys_returns_multiple_keys() {
    let context = TestContext::new();
    let executor = start(&context).await.unwrap();

    let component_id = executor.store_component("key-value-service").await;
    let worker_name = "key-value-service-13";
    let worker_id = executor.start_worker(&component_id, worker_name).await;

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key1".to_string()),
                Value::List(vec![Value::U8(1), Value::U8(2), Value::U8(3)]),
            ],
        )
        .await
        .unwrap();

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key2".to_string()),
                Value::List(vec![Value::U8(4), Value::U8(5), Value::U8(6)]),
            ],
        )
        .await
        .unwrap();

    let _ = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/set",
            vec![
                Value::String(format!("{component_id}-{worker_name}-bucket")),
                Value::String("key3".to_string()),
                Value::List(vec![Value::U8(7), Value::U8(8), Value::U8(9)]),
            ],
        )
        .await
        .unwrap();

    let result = executor
        .invoke_and_await(
            &worker_id,
            "golem:it/api/get-keys",
            vec![Value::String(format!(
                "{component_id}-{worker_name}-bucket"
            ))],
        )
        .await
        .unwrap();

    drop(executor);

    check!(
        result
            == vec![Value::List(vec![
                Value::String("key1".to_string()),
                Value::String("key2".to_string()),
                Value::String("key3".to_string()),
            ])]
    );
}
