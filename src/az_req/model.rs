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
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyStore {
    pub kind: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entities {
    pub schema: String,
    pub items: Vec<Option<HashMap<String, Value>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evaluation {
    pub request_id: String,
    pub subject: Option<Subject>,
    pub resource: Option<Resource>,
    pub action: Option<Action>,
    pub context: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzModel {
    pub zone_id: i64,
    pub principal: Option<Principal>,
    pub policy_store: Option<PolicyStore>,
    pub entities: Option<Entities>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzRequest {
    pub authorization_model: AzModel,
    pub request_id: Option<String>,
    pub subject: Option<Subject>,
    pub resource: Option<Resource>,
    pub action: Option<Action>,
    pub context: Option<HashMap<String, Value>>,
    pub evaluations: Option<Vec<Evaluation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal {
    pub r#type: String,
    pub id: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub r#type: String,
    pub id: String,
    pub source: Option<String>,
    pub properties: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub r#type: String,
    pub id: String,
    pub properties: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub properties: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonResponse {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextResponse {
    pub id: String,
    pub reason_admin: Option<ReasonResponse>,
    pub reason_user: Option<ReasonResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResponse {
    pub request_id: String,
    pub decision: bool,
    pub context: Option<ContextResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzResponse {
    pub request_id: String,
    pub decision: bool,
    pub context: Option<ContextResponse>,
    pub evaluations: Vec<EvaluationResponse>,
}
