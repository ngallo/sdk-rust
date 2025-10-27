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

use crate::az_req::model::Principal;

/// Default kind for principals.
pub mod principal_default {
    pub const USER_TYPE: &str = "user";
}

/// Builder for [`Principal`].
#[derive(Debug, Clone)]
pub struct PrincipalBuilder {
    id: String,
    principal_type: String,
    source: Option<String>,
}

impl PrincipalBuilder {
    /// Creates a new `PrincipalBuilder` with the given ID and default type `"user"`.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            principal_type: principal_default::USER_TYPE.to_string(),
            source: None,
        }
    }

    /// Sets the kind/type of the principal.
    pub fn with_kind(mut self, kind: impl Into<String>) -> Self {
        self.principal_type = kind.into();
        self
    }

    /// Sets the source of the principal.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Builds and returns the final [`Principal`] instance.
    pub fn build(self) -> Principal {
        Principal {
            id: self.id,
            r#type: self.principal_type,
            source: self.source,
        }
    }
}
