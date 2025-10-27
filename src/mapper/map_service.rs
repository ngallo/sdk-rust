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

use std::collections::{HashMap, BTreeMap};
use prost_types::{Struct, Value, ListValue};
use crate::az_client::policydecisionpoint;
use crate::az_req::model::*;

pub struct MapService;

impl MapService {
    // ------------------------------------
    // From Dictionary to Struct
    // ------------------------------------
    pub fn from_dictionary(dict: Option<&HashMap<String, serde_json::Value>>) -> Option<Struct> {
        let mut fields = BTreeMap::new();
        if let Some(map) = dict {
            for (key, val) in map {
                fields.insert(key.clone(), Self::convert_to_value(val));
            }
        }
        Some(Struct { fields })
    }

    // Convert list of dictionaries into list of protobuf Structs
    pub fn to_repeated_struct(items: Option<&Vec<Option<HashMap<String, serde_json::Value>>>>) -> Vec<Struct> {
        let mut list = Vec::new();
        if let Some(vec) = items {
            for item in vec {
                if let Some(map) = item {
                    if let Some(s) = Self::from_dictionary(Some(map)) {
                        list.push(s);
                    }
                }
            }
        }
        list
    }

    // ------------------------------------
    // Convert serde_json::Value to protobuf Value
    // ------------------------------------
    fn convert_to_value(v: &serde_json::Value) -> Value {
        match v {
            serde_json::Value::Null => Value { kind: Some(prost_types::value::Kind::NullValue(0)) },
            serde_json::Value::Bool(b) => Value { kind: Some(prost_types::value::Kind::BoolValue(*b)) },
            serde_json::Value::Number(n) => {
                let f = n.as_f64().unwrap_or_default();
                Value { kind: Some(prost_types::value::Kind::NumberValue(f)) }
            }
            serde_json::Value::String(s) => Value { kind: Some(prost_types::value::Kind::StringValue(s.clone())) },
            serde_json::Value::Array(arr) => {
                let values: Vec<Value> = arr.iter().map(Self::convert_to_value).collect();
                Value {
                    kind: Some(prost_types::value::Kind::ListValue(ListValue { values })),
                }
            }
            serde_json::Value::Object(map) => {
                let struct_val = Struct {
                    fields: map.iter().map(|(k, v)| (k.clone(), Self::convert_to_value(v))).collect(),
                };
                Value {
                    kind: Some(prost_types::value::Kind::StructValue(struct_val)),
                }
            }
        }
    }

    // ------------------------------------
    // Convert Struct to Dictionary
    // ------------------------------------
    pub fn to_dictionary(s: &Struct) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        for (k, v) in &s.fields {
            map.insert(k.clone(), Self::convert_from_value(v));
        }
        map
    }

    fn convert_from_value(v: &Value) -> serde_json::Value {
        match &v.kind {
            Some(prost_types::value::Kind::NullValue(_)) => serde_json::Value::Null,
            Some(prost_types::value::Kind::BoolValue(b)) => serde_json::Value::Bool(*b),
            Some(prost_types::value::Kind::NumberValue(f)) => serde_json::json!(f),
            Some(prost_types::value::Kind::StringValue(s)) => serde_json::Value::String(s.clone()),
            Some(prost_types::value::Kind::StructValue(st)) => {
                serde_json::Value::Object(Self::to_dictionary(st).into_iter().collect())
            }
            Some(prost_types::value::Kind::ListValue(list)) => {
                serde_json::Value::Array(list.values.iter().map(Self::convert_from_value).collect())
            }
            None => serde_json::Value::Null,
        }
    }

    // ------------------------------------
    // Mapping AzReq → gRPC
    // ------------------------------------
    fn map_policy_store(ps: &Option<PolicyStore>) -> Option<policydecisionpoint::PolicyStore> {
        ps.as_ref().map(|p| policydecisionpoint::PolicyStore {
            id: p.id.clone(),
            kind: p.kind.clone(),
        })
    }

    fn map_principal(pr: &Option<Principal>) -> Option<policydecisionpoint::Principal> {
        pr.as_ref().map(|p| policydecisionpoint::Principal {
            id: p.id.clone(),
            r#type: p.r#type.clone(),
            source: p.source.clone(),
            identity_token: None,
            access_token: None,
        })
    }

    fn map_entities(ent: &Option<Entities>) -> Option<policydecisionpoint::Entities> {
        ent.as_ref().map(|e| {
            let items = Self::to_repeated_struct(Some(&e.items));
            policydecisionpoint::Entities {
                schema: e.schema.clone(),
                items,
            }
        })
    }

    fn map_subject(sub: &Option<Subject>) -> Option<policydecisionpoint::Subject> {
        sub.as_ref().map(|s| policydecisionpoint::Subject {
            id: s.id.clone(),
            r#type: s.r#type.clone(),
            source: s.source.clone(),
            properties: Self::from_dictionary(s.properties.as_ref()),
        })
    }

    fn map_resource(res: &Option<Resource>) -> Option<policydecisionpoint::Resource> {
        res.as_ref().map(|r| policydecisionpoint::Resource {
            id: r.id.clone(),
            r#type: r.r#type.clone(),
            properties: Self::from_dictionary(r.properties.as_ref()),
        })
    }

    fn map_action(act: &Option<Action>) -> Option<policydecisionpoint::Action> {
        act.as_ref().map(|a| policydecisionpoint::Action {
            name: a.name.clone(),
            properties: Self::from_dictionary(a.properties.as_ref()),
        })
    }

    fn map_evaluation(eval: &Option<Evaluation>) -> Option<policydecisionpoint::EvaluationRequest> {
        eval.as_ref().map(|e| policydecisionpoint::EvaluationRequest {
            request_id: Some(e.request_id.clone()),
            subject: Self::map_subject(&e.subject),
            resource: Self::map_resource(&e.resource),
            action: Self::map_action(&e.action),
            context: Self::from_dictionary(e.context.as_ref()),
        })
    }

    fn map_authz_model(model: &AzModel) -> policydecisionpoint::AuthorizationModelRequest {
        policydecisionpoint::AuthorizationModelRequest {
            zone_id: model.zone_id,
            policy_store: Self::map_policy_store(&model.policy_store),
            principal: Self::map_principal(&model.principal),
            entities: Self::map_entities(&model.entities),
        }
    }

    pub fn map_az_request(req: &AzRequest) -> policydecisionpoint::AuthorizationCheckRequest {
        let mut grpc_req = policydecisionpoint::AuthorizationCheckRequest {
            request_id: req.request_id.clone(),
            authorization_model: Some(Self::map_authz_model(&req.authorization_model)),
            subject: Self::map_subject(&req.subject),
            resource: Self::map_resource(&req.resource),
            action: Self::map_action(&req.action),
            context: Self::from_dictionary(req.context.as_ref()),
            evaluations: Vec::new(),
        };

        if let Some(evs) = &req.evaluations {
            for e in evs {
                if let Some(ev) = Self::map_evaluation(&Some(e.clone())) {
                    grpc_req.evaluations.push(ev);
                }
            }
        }

        grpc_req
    }

    // ------------------------------------
    // Mapping gRPC → AzResponse
    // ------------------------------------
    fn map_reason_response(r: &Option<policydecisionpoint::ReasonResponse>) -> Option<ReasonResponse> {
        r.as_ref().map(|rr| ReasonResponse {
            code: rr.code.clone(),
            message: rr.message.clone(),
        })
    }

    fn map_context_response(c: &Option<policydecisionpoint::ContextResponse>) -> Option<ContextResponse> {
        c.as_ref().map(|ctx| ContextResponse {
            id: ctx.id.clone(),
            reason_admin: Self::map_reason_response(&ctx.reason_admin),
            reason_user: Self::map_reason_response(&ctx.reason_user),
        })
    }

    fn map_evaluation_response(e: &Option<policydecisionpoint::EvaluationResponse>) -> Option<EvaluationResponse> {
        e.as_ref().map(|ev| EvaluationResponse {
            request_id: ev.request_id.clone().unwrap_or_default(),
            decision: ev.decision,
            context: Self::map_context_response(&ev.context),
        })
    }

    pub fn map_grpc_response(resp: &policydecisionpoint::AuthorizationCheckResponse) -> AzResponse {
        AzResponse {
            request_id: resp.request_id.clone().unwrap_or_default(),
            decision: resp.decision,
            context: Self::map_context_response(&resp.context),
            evaluations: resp
                .evaluations
                .iter()
                .filter_map(|e| Self::map_evaluation_response(&Some(e.clone())))
                .collect(),
        }
    }
}
