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
use crate::az_req::action_builder::ActionBuilder;
use crate::az_req::az_request_builder::AzRequestBuilder;
use crate::az_req::context_builder::ContextBuilder;
use crate::az_req::model::*;
use crate::az_req::resource_builder::ResourceBuilder;
use crate::az_req::subject_builder::SubjectBuilder;

/// Builder for constructing a complete [`AzRequest`] in one step.
///
/// This builder composes all other builders:
/// - [`SubjectBuilder`]
/// - [`ResourceBuilder`]
/// - [`ActionBuilder`]
/// - [`ContextBuilder`]
/// - [`AzRequestBuilder`]
#[derive(Debug)]
pub struct AzAtomicRequestBuilder {
    request_id: Option<String>,
    principal: Option<Principal>,
    az_subject_builder: SubjectBuilder,
    az_resource_builder: ResourceBuilder,
    az_action_builder: ActionBuilder,
    az_context_builder: ContextBuilder,
    az_request_builder: AzRequestBuilder,
}

impl AzAtomicRequestBuilder {
    /// Creates a new atomic builder.
    pub fn new(
        zone_id: i64,
        ledger_id: impl Into<String>,
        subject_id: impl Into<String>,
        resource_kind: impl Into<String>,
        action_name: impl Into<String>,
    ) -> Self {
        Self {
            request_id: None,
            principal: None,
            az_subject_builder: SubjectBuilder::new(subject_id),
            az_resource_builder: ResourceBuilder::new(resource_kind),
            az_action_builder: ActionBuilder::new(action_name),
            az_context_builder: ContextBuilder::new(),
            az_request_builder: AzRequestBuilder::new(zone_id, ledger_id),
        }
    }

    /// Sets the entities map.
    pub fn with_entities_map(
        mut self,
        schema: impl Into<String>,
        entities: Vec<Option<HashMap<String, Value>>>,
    ) -> Self {
        self.az_request_builder = self.az_request_builder.with_entities_map(schema, entities);
        self
    }

    /// Sets entities items.
    pub fn with_entities_items(
        mut self,
        schema: impl Into<String>,
        entities: Option<Vec<Option<HashMap<String, Value>>>>,
    ) -> Self {
        self.az_request_builder = self.az_request_builder.with_entities_items(schema, entities);
        self
    }

    /// Sets the request ID.
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// Sets the principal.
    pub fn with_principal(mut self, principal: Principal) -> Self {
        self.principal = Some(principal);
        self
    }

    /// Sets the subject kind.
    pub fn with_subject_kind(mut self, kind: impl Into<String>) -> Self {
        self.az_subject_builder = self.az_subject_builder.with_type(kind);
        self
    }

    /// Sets the subject source.
    pub fn with_subject_source(mut self, source: impl Into<String>) -> Self {
        self.az_subject_builder = self.az_subject_builder.with_source(source);
        self
    }

    /// Sets a subject property.
    pub fn with_subject_property(mut self, key: impl Into<String>, value: Value) -> Self {
        self.az_subject_builder = self.az_subject_builder.with_property(key, value);
        self
    }

    /// Sets the resource ID.
    pub fn with_resource_id(mut self, id: impl Into<String>) -> Self {
        self.az_resource_builder = self.az_resource_builder.with_id(id);
        self
    }

    /// Sets a resource property.
    pub fn with_resource_property(mut self, key: impl Into<String>, value: Value) -> Self {
        self.az_resource_builder = self.az_resource_builder.with_property(key, value);
        self
    }

    /// Sets an action property.
    pub fn with_action_property(mut self, key: impl Into<String>, value: Value) -> Self {
        self.az_action_builder = self.az_action_builder.with_property(key, value);
        self
    }

    /// Sets a context property.
    pub fn with_context_property(mut self, key: impl Into<String>, value: Value) -> Self {
        self.az_context_builder = self.az_context_builder.with_property(key, value);
        self
    }

    /// Builds and returns the [`AzRequest`].
    pub fn build(self) -> AzRequest {
        let subject = self.az_subject_builder.build();
        let resource = self.az_resource_builder.build();
        let action = self.az_action_builder.build();
        let context = self.az_context_builder.build();

        self.az_request_builder
            .with_principal(self.principal)
            .with_request_id(self.request_id)
            .with_subject(Some(subject))
            .with_resource(Some(resource))
            .with_action(Some(action))
            .with_context(Some(context))
            .build()
    }
}
