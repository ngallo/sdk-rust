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
use crate::az_req::model::Subject;

/// Default subject constants.
pub mod subject_default {
    pub const USER_TYPE: &str = "user";
}

/// Builder for [`Subject`].
#[derive(Debug, Clone)]
pub struct SubjectBuilder {
    id: String,
    subject_type: String,
    source: Option<String>,
    properties: HashMap<String, Value>,
}

impl SubjectBuilder {
    /// Creates a new `SubjectBuilder` with a given subject ID.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            subject_type: subject_default::USER_TYPE.to_string(),
            source: None,
            properties: HashMap::new(),
        }
    }

    /// Sets the type of the subject.
    pub fn with_type(mut self, kind: impl Into<String>) -> Self {
        self.subject_type = kind.into();
        self
    }

    /// Sets the source of the subject.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Adds a property key/value pair to the subject.
    pub fn with_property(mut self, key: impl Into<String>, value: Value) -> Self {
        self.properties.insert(key.into(), value);
        self
    }

    /// Builds the final [`Subject`] instance.
    pub fn build(self) -> Subject {
        Subject {
            id: self.id,
            r#type: self.subject_type,
            source: self.source,
            properties: Some(self.properties),
        }
    }
}
