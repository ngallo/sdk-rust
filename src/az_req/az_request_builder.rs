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

use serde_json::Value;
use std::collections::HashMap;
use crate::az_req::builder::deep_copy;
use crate::az_req::model::{Action, AzModel, AzRequest, Entities, Evaluation, PolicyStore, Principal, Resource, Subject};

/// Builder for constructing an [`AzRequest`] instance.
#[derive(Debug, Clone)]
pub struct AzRequestBuilder {
    az_request: AzRequest,
}

impl AzRequestBuilder {
    /// Initializes a new [`AzRequestBuilder`] with `zone_id` and `ledger_id`.
    pub fn new(zone_id: i64, ledger_id: impl Into<String>) -> Self {
        let az_request = AzRequest {
            authorization_model: AzModel {
                zone_id,
                principal: None,
                policy_store: Some(PolicyStore {
                    kind: "ledger".to_string(),
                    id: ledger_id.into(),
                }),
                entities: Some(Entities {
                    schema: "".to_string(),
                    items: Vec::new(),
                }),
            },
            request_id: None,
            subject: None,
            resource: None,
            action: None,
            context: None,
            evaluations: None,
        };

        Self { az_request }
    }

    /// Sets the principal.
    pub fn with_principal(mut self, principal: Option<Principal>) -> Self {
        self.az_request.authorization_model.principal = principal;
        self
    }

    /// Sets the request ID.
    pub fn with_request_id(mut self, request_id: Option<String>) -> Self {
        self.az_request.request_id = request_id;
        self
    }

    /// Sets the subject.
    pub fn with_subject(mut self, subject: Option<Subject>) -> Self {
        self.az_request.subject = subject;
        self
    }

    /// Sets the resource.
    pub fn with_resource(mut self, resource: Option<Resource>) -> Self {
        self.az_request.resource = resource;
        self
    }

    /// Sets the action.
    pub fn with_action(mut self, action: Option<Action>) -> Self {
        self.az_request.action = action;
        self
    }

    /// Sets the context.
    pub fn with_context(mut self, context: Option<HashMap<String, Value>>) -> Self {
        self.az_request.context = context.map(|ctx| deep_copy(Some(&ctx)));
        self
    }

    /// Sets entities using a schema and list of entity maps.
    pub fn with_entities_map(
        mut self,
        schema: impl Into<String>,
        entities: Vec<Option<HashMap<String, Value>>>,
    ) -> Self {
        match &mut self.az_request.authorization_model.entities {
            Some(existing) => {
                existing.schema = schema.into();
                existing.items = entities;
            }
            None => {
                self.az_request.authorization_model.entities = Some(Entities {
                    schema: schema.into(),
                    items: entities,
                });
            }
        }
        self
    }

    /// Sets entities items directly.
    pub fn with_entities_items(
        mut self,
        schema: impl Into<String>,
        entities: Option<Vec<Option<HashMap<String, Value>>>>,
    ) -> Self {
        match &mut self.az_request.authorization_model.entities {
            Some(existing) => {
                existing.schema = schema.into();
                existing.items = entities.unwrap_or_default();
            }
            None => {
                self.az_request.authorization_model.entities = Some(Entities {
                    schema: schema.into(),
                    items: entities.unwrap_or_default(),
                });
            }
        }
        self
    }

    /// Adds an evaluation.
    pub fn with_evaluation(mut self, evaluation: Evaluation) -> Self {
        self.az_request.evaluations.get_or_insert_with(Vec::new).push(evaluation);
        self
    }

    /// Builds the [`AzRequest`] object.
    pub fn build(self) -> AzRequest {
        self.az_request
    }
}
