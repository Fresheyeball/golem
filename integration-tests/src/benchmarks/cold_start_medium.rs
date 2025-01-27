// Copyright 2024 Golem Cloud
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use async_trait::async_trait;
use golem_common::model::WorkerId;

use golem_test_framework::config::{CliParams, TestDependencies};
use golem_test_framework::dsl::benchmark::{Benchmark, BenchmarkRecorder, RunConfig};
use integration_tests::benchmarks::{
    cleanup_iteration, get_worker_ids, run_benchmark, run_echo, setup_benchmark, setup_iteration,
    start, BenchmarkContext, IterationContext,
};

struct ColdStartEchoMedium {
    config: RunConfig,
}

#[async_trait]
impl Benchmark for ColdStartEchoMedium {
    type BenchmarkContext = BenchmarkContext;
    type IterationContext = IterationContext;

    fn name() -> &'static str {
        "cold-start-medium"
    }

    async fn create_benchmark_context(
        params: CliParams,
        cluster_size: usize,
    ) -> Self::BenchmarkContext {
        setup_benchmark(params, cluster_size).await
    }

    async fn cleanup(benchmark_context: Self::BenchmarkContext) {
        benchmark_context.deps.kill_all()
    }

    async fn create(_params: CliParams, config: RunConfig) -> Self {
        Self { config }
    }

    async fn setup_iteration(
        &self,
        benchmark_context: &Self::BenchmarkContext,
    ) -> Self::IterationContext {
        setup_iteration(benchmark_context, self.config.clone(), "js-echo", false).await
    }

    async fn warmup(
        &self,
        benchmark_context: &Self::BenchmarkContext,
        context: &Self::IterationContext,
    ) {
        // warmup with other workers
        if let Some(WorkerId { component_id, .. }) = context.worker_ids.clone().first() {
            start(
                get_worker_ids(context.worker_ids.len(), component_id, "warmup-worker"),
                benchmark_context.deps.clone(),
            )
            .await
        }
    }

    async fn run(
        &self,
        benchmark_context: &Self::BenchmarkContext,
        context: &Self::IterationContext,
        recorder: BenchmarkRecorder,
    ) {
        // config.benchmark_config.length is not used, we want to have only one invocation per worker in this benchmark
        run_echo(1, benchmark_context, context, recorder).await
    }

    async fn cleanup_iteration(
        &self,
        benchmark_context: &Self::BenchmarkContext,
        context: Self::IterationContext,
    ) {
        cleanup_iteration(benchmark_context, context).await
    }
}

#[tokio::main]
async fn main() {
    run_benchmark::<ColdStartEchoMedium>().await;
}
