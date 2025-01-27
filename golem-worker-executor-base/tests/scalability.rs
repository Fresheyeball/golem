use crate::common::{start, TestContext, TestWorkerExecutor};
use assert2::check;
use futures_util::stream::FuturesUnordered;
use futures_util::StreamExt;
use golem_common::model::ComponentId;
use golem_test_framework::dsl::TestDsl;
use golem_wasm_rpc::Value;
use std::future::Future;
use std::time::Duration;
use tokio::spawn;
use tracing::info;

#[tokio::test(flavor = "multi_thread")]
#[tracing::instrument]
async fn spawning_many_workers_that_sleep() {
    let context = TestContext::new();
    fn worker_name(n: i32) -> String {
        format!("sleeping-worker-{}", n)
    }

    async fn timed<F>(f: F) -> (F::Output, Duration)
    where
        F: Future + Send + 'static,
    {
        let start = tokio::time::Instant::now();
        let result = f.await;
        let duration = start.elapsed();
        (result, duration)
    }

    let executor = start(&context).await.unwrap();
    let component_id = executor.store_component("clocks").await;

    let warmup_worker = executor.start_worker(&component_id, &worker_name(0)).await;

    let executor_clone = executor.clone();
    let warmup_result = timed(async move {
        executor_clone
            .invoke_and_await(&warmup_worker, "run", vec![])
            .await
            .unwrap()
    })
    .await;

    info!("Warmup: {:?}", warmup_result);

    const N: i32 = 100;
    info!("{N} instances");

    let start = tokio::time::Instant::now();
    let input: Vec<(i32, ComponentId, TestWorkerExecutor)> = (1..N)
        .map(|i| (i, component_id.clone(), executor.clone()))
        .collect();
    let fibers: Vec<_> = input
        .into_iter()
        .map(|(n, component_id, executor_clone)| {
            spawn(async move {
                let worker = executor_clone
                    .start_worker(&component_id, &worker_name(n))
                    .await;
                timed(async move {
                    executor_clone
                        .invoke_and_await(&worker, "run", vec![])
                        .await
                        .unwrap()
                })
                .await
            })
        })
        .collect();

    info!("Spawned all, waiting...");
    let futures: FuturesUnordered<_> = fibers.into_iter().collect();
    let results: Vec<_> = futures.collect().await;

    let total_duration = start.elapsed();

    info!("Results: {:?}", results);
    info!("Total duration: {:?}", total_duration);

    let mut sorted = results
        .into_iter()
        .map(|r| match r {
            Ok((_, duration)) => duration.as_millis(),
            Err(err) => panic!("Error: {:?}", err),
        })
        .collect::<Vec<_>>();
    sorted.sort();
    let idx = (sorted.len() as f64 * 0.95) as usize;
    let p95 = sorted[idx];

    drop(executor);

    check!(p95 < 6000);
    check!(total_duration.as_secs() < 10);
}

#[tokio::test(flavor = "multi_thread")]
#[tracing::instrument]
async fn spawning_many_workers_that_sleep_long_enough_to_get_suspended() {
    let context = TestContext::new();
    fn worker_name(n: i32) -> String {
        format!("sleeping-suspending-worker-{}", n)
    }

    async fn timed<F>(f: F) -> (F::Output, Duration)
    where
        F: Future + Send + 'static,
    {
        let start = tokio::time::Instant::now();
        let result = f.await;
        let duration = start.elapsed();
        (result, duration)
    }

    let executor = start(&context).await.unwrap();
    let component_id = executor.store_component("clocks").await;

    let warmup_worker = executor.start_worker(&component_id, &worker_name(0)).await;

    let executor_clone = executor.clone();
    let warmup_result = timed(async move {
        executor_clone
            .invoke_and_await(&warmup_worker, "sleep-for", vec![Value::F64(15.0)])
            .await
            .unwrap()
    })
    .await;

    info!("Warmup: {:?}", warmup_result);

    const N: i32 = 100;
    info!("{N} instances");

    let start = tokio::time::Instant::now();
    let input: Vec<(i32, ComponentId, TestWorkerExecutor)> = (1..N)
        .map(|i| (i, component_id.clone(), executor.clone()))
        .collect();
    let fibers: Vec<_> = input
        .into_iter()
        .map(|(n, component_id, executor_clone)| {
            spawn(async move {
                let worker = executor_clone
                    .start_worker(&component_id, &worker_name(n))
                    .await;
                timed(async move {
                    executor_clone
                        .invoke_and_await(&worker, "sleep-for", vec![Value::F64(15.0)])
                        .await
                        .unwrap()
                })
                .await
            })
        })
        .collect();

    info!("Spawned all, waiting...");
    let futures: FuturesUnordered<_> = fibers.into_iter().collect();
    let results: Vec<_> = futures.collect().await;

    let total_duration = start.elapsed();

    info!("Results: {:?}", results);
    info!("Total duration: {:?}", total_duration);

    let mut sorted1 = results
        .iter()
        .map(|r| match r {
            Ok((r, _)) => {
                let Value::F64(seconds) = r[0] else {
                    panic!("Unexpected result")
                };
                (seconds * 1000.0) as u64
            }
            Err(err) => panic!("Error: {:?}", err),
        })
        .collect::<Vec<_>>();
    sorted1.sort();
    let idx1 = (sorted1.len() as f64 * 0.95) as usize;
    let p951 = sorted1[idx1];

    let mut sorted2 = results
        .into_iter()
        .map(|r| match r {
            Ok((_, duration)) => duration.as_millis(),
            Err(err) => panic!("Error: {:?}", err),
        })
        .collect::<Vec<_>>();
    sorted2.sort();
    let idx2 = (sorted2.len() as f64 * 0.95) as usize;
    let p952 = sorted2[idx2];

    drop(executor);

    check!(p951 < 25000);
    check!(p952 < 25000);
}
