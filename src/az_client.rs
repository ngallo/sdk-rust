// Copyright 2025 Nitro Agility S.r.l.
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
//
// SPDX-License-Identifier: Apache-2.0

use tonic::transport::Channel;
use tonic::Request;
use crate::config::AzConfig;
use crate::az_req::*;
use crate::mapper::MapService;

pub mod policydecisionpoint {
    tonic::include_proto!("policydecisionpoint");
}


pub struct AzClient {
    config: AzConfig,
}

impl AzClient {
    pub fn new(config: AzConfig) -> Self {
        Self { config }
    }

    /// Perform an authorization check via gRPC.
    pub async fn check_auth(&self, request: Option<model::AzRequest>) -> Result<model::AzResponse, Box<dyn std::error::Error>> {
        let endpoint = self
            .config
            .endpoint
            .as_ref()
            .ok_or("Please provide config")?;

        let url = format!("{}://{}:{}", endpoint.schema, endpoint.host, endpoint.port);

        let channel = Channel::from_shared(url.clone())?
            .connect()
            .await?;

        let mut client = policydecisionpoint::v1pdp_service_client::V1pdpServiceClient::new(channel);

        let grpc_request: policydecisionpoint::AuthorizationCheckRequest = match request {
            Some(ref req) => MapService::map_az_request(req),
            None => return Err("Invalid AzRequest".into()),
        };

        let response = client
            .authorization_check(Request::new(grpc_request))
            .await?
            .into_inner();

        let mapped_response = MapService::map_grpc_response(&response);

        Ok(mapped_response)
    }
}
