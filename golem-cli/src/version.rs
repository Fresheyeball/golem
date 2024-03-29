
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
use version_compare::Version;

use crate::clients::health_check::HealthCheckClient;
use crate::model::{
    GolemError, GolemResult
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[async_trait]
pub trait VersionHandler {
    async fn check(&self) -> Result<GolemResult, GolemError>;
}

pub struct VersionHandlerLive<T: HealthCheckClient + Send + Sync + Send + Sync, W: HealthCheckClient + Send + Sync + Send + Sync> {
    pub template_client: T,
    pub worker_client: W,
}

#[async_trait]
impl<T: HealthCheckClient + Send + Sync, W: HealthCheckClient + Send + Sync> VersionHandler
    for VersionHandlerLive<T, W>
{
    async fn check(&self) -> Result<GolemResult, GolemError> {
        let template_version_info = self.template_client.version().await?;
        let worker_version_info = self.worker_client.version().await?;
        
        let cli_version = Version::from(VERSION).unwrap();
        let template_version = Version::from(template_version_info.version.as_str()).unwrap();
        let worker_version = Version::from(worker_version_info.version.as_str()).unwrap();

        let warning = |cli_version: Version, server_version: Version| -> String {
            format!("Warning: golem-cli {} is older than the targeted Golem servers ({})\nInstall the matching version with:\ncargo install golem-cli@{}\n", cli_version.as_str(), server_version.as_str(), server_version.as_str()).to_string()
        };
        
        if cli_version < template_version || cli_version < worker_version {
            if template_version > worker_version {
                Err(GolemError(warning(cli_version, template_version)))
            } else {
                Err(GolemError(warning(cli_version, worker_version)))
            }
        } else if cli_version < template_version {
            Err(GolemError(warning(cli_version, template_version)))
        } else if cli_version < worker_version {
            Err(GolemError(warning(cli_version, worker_version)))
        } else {
            Ok(GolemResult::Str("No updates found".to_string()))
        }
    }
}