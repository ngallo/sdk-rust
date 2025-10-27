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

/// Builder for creating a context represented as a `HashMap<String, Value>`.
#[derive(Debug, Default, Clone)]
pub struct ContextBuilder {
    context: HashMap<String, Value>,
}

impl ContextBuilder {
    /// Creates a new empty `ContextBuilder`.
    pub fn new() -> Self {
        Self {
            context: HashMap::new(),
        }
    }

    /// Adds or updates a property in the context.
    pub fn with_property(mut self, key: impl Into<String>, value: Value) -> Self {
        self.context.insert(key.into(), value);
        self
    }

    /// Builds and returns the final context map.
    pub fn build(self) -> HashMap<String, Value> {
        self.context
    }
}
