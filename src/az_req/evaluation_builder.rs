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
use crate::az_req::model::{Action, Evaluation, Resource, Subject};

/// Builder for creating an [`Evaluation`] instance.
#[derive(Debug, Clone)]
pub struct EvaluationBuilder {
    evaluation: Evaluation,
}

impl EvaluationBuilder {
    /// Creates a new [`EvaluationBuilder`] with optional subject, resource, and action.
    pub fn new(subject: Option<Subject>, resource: Option<Resource>, action: Option<Action>) -> Self {
        Self {
            evaluation: Evaluation {
                request_id: String::new(),
                subject,
                resource,
                action,
                context: None,
            },
        }
    }

    /// Sets the `request_id` of the evaluation.
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.evaluation.request_id = request_id.into();
        self
    }

    /// Sets the `context` of the evaluation.
    pub fn with_context(mut self, context: Option<HashMap<String, Value>>) -> Self {
        self.evaluation.context = context;
        self
    }

    /// Builds and returns the final [`Evaluation`] object.
    pub fn build(self) -> Evaluation {
        self.evaluation
    }
}
