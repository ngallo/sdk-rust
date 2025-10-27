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

use std::collections::HashMap;
use serde_json::Value;
use crate::az_req::model::Resource;

/// Builder for [`Resource`].
#[derive(Debug, Clone)]
pub struct ResourceBuilder {
    id: Option<String>,
    resource_type: String,
    properties: HashMap<String, Value>,
}

impl ResourceBuilder {
    /// Creates a new `ResourceBuilder` with a given type (kind).
    pub fn new(kind: impl Into<String>) -> Self {
        Self {
            id: None,
            resource_type: kind.into(),
            properties: HashMap::new(),
        }
    }

    /// Sets the ID of the resource.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Adds a property key/value pair to the resource.
    pub fn with_property(mut self, key: impl Into<String>, value: Value) -> Self {
        self.properties.insert(key.into(), value);
        self
    }

    /// Builds and returns the final [`Resource`] instance.
    pub fn build(self) -> Resource {
        Resource {
            id: self.id.unwrap_or_else(|| "".to_string()),
            r#type: self.resource_type,
            properties: Some(self.properties),
        }
    }
}
