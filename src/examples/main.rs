mod config;
mod az_client;
mod az_req;
mod mapper;

use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use crate::az_client::AzClient;
use crate::az_req::action_builder::ActionBuilder;
use crate::az_req::az_request_builder::AzRequestBuilder;
use crate::az_req::context_builder::ContextBuilder;
use crate::az_req::evaluation_builder::EvaluationBuilder;
use crate::az_req::model::AzRequest;
use crate::az_req::principal_builder::PrincipalBuilder;
use crate::az_req::resource_builder::ResourceBuilder;
use crate::az_req::subject_builder::SubjectBuilder;
use crate::config::{AzConfig, AzEndpoint};
use tokio::fs;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error>> {
    /*
    match first_test().await {
        Ok(value) => value,
        Err(value) => return value,
    }*/
    match test_json().await {
        Ok(value) => value,
        Err(value) => return value,
    }
}


async fn test_json() -> Result<Result<(), Box<dyn Error>>, Result<(), Box<dyn Error>>>{
    let endpoint = AzEndpoint::new("http".to_string(), 9094, "localhost".to_string());
    let config = AzConfig::new().with_endpoint(Some(endpoint));
    let client = AzClient::new(config);

    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("../json/ok_onlyone.json");

    if !file_path.exists() {
        eprintln!("❌ Failed to load the JSON file");
        return Err(Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Failed to load the JSON file",
        ))));
    }

    let json_content = fs::read_to_string(&file_path).await;
    let request: AzRequest = match serde_json::from_str(&json_content.unwrap()) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("❌ Failed to parse JSON: {}", e);
            return Err(Err(Box::new(e) as Box<dyn std::error::Error>));
        }
    };

    match client.check_auth(Some(request)).await {
        Ok(response) => {
            if response.decision {
                println!("✅ Authorization Permitted");
            } else {
                println!("❌ Authorization Denied");
                if let Some(ctx) = response.context {
                    if let Some(admin) = ctx.reason_admin {
                        println!("-> Reason Admin: {}", admin.message);
                    }
                    if let Some(user) = ctx.reason_user {
                        println!("-> Reason User: {}", user.message);
                    }
                }

                for eval in response.evaluations {
                    if eval.decision {
                        println!("-> ✅ Authorization Permitted");
                    }
                    if let Some(ctx) = eval.context {
                        if let Some(admin) = ctx.reason_admin {
                            println!("-> Reason Admin: {}", admin.message);
                        }
                        if let Some(user) = ctx.reason_user {
                            println!("-> Reason User: {}", user.message);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to check auth: {}", e);
            return Err(Err(e.into()));
        }
    }

    Ok(Ok(()))
}

async fn first_test() -> Result<Result<(), Box<dyn Error>>, Result<(), Box<dyn Error>>> {
    let endpoint = AzEndpoint::new("http".to_string(), 9094, "localhost".to_string());
    let config = AzConfig::new().with_endpoint(Some(endpoint));
    let client = AzClient::new(config);

    // Create the Principal
    let principal = PrincipalBuilder::new("amy.smith@acmecorp.com".to_string())
        .with_source("keycloak".to_string())
        .with_kind("user".to_string())
        .build();

    // Create a new subject
    let subject = SubjectBuilder::new("platform-creator".to_string())
        .with_source("keycloak".to_string())
        .with_kind("role-actor".to_string())
        .with_property("isSuperUser".to_string(), serde_json::json!(true))
        .build();

    // Create a new resource
    let resource = ResourceBuilder::new("MagicFarmacia::Platform::Subscription".to_string())
        .with_id("e3a786fd07e24bfa95ba4341d3695ae8".to_string())
        .with_property("isEnabled".to_string(), serde_json::json!(true))
        .build();

    // Create actions
    let action_view = ActionBuilder::new("MagicFarmacia::Platform::Action::create".to_string())
        .with_property("isEnabled".to_string(), serde_json::json!(true))
        .build();

    let action_create = ActionBuilder::new("MagicFarmacia::Platform::Action::create".to_string())
        .with_property("isEnabled".to_string(), serde_json::json!(false))
        .build();

    // Create a new Context
    let context = ContextBuilder::new()
        .with_property("time".to_string(), serde_json::json!("2025-01-23T16:17:46+00:00"))
        .with_property("isSubscriptionActive".to_string(), serde_json::json!(true))
        .build();

    // Create evaluations
    let evaluation_view = EvaluationBuilder::new(Some(subject.clone()), Some(resource.clone()), Some(action_view.clone()))
        .with_request_id("134".to_string())
        .build();

    let evaluation_create = EvaluationBuilder::new(Some(subject.clone()), Some(resource.clone()), Some(action_create.clone()))
        .with_request_id("435".to_string())
        .build();

    // Create the entities
    let mut entity = HashMap::new();
    entity.insert(
        "uid".to_string(),
        serde_json::json!({
            "type": "MagicFarmacia::Platform::BranchInfo",
            "id": "subscription"
        }),
    );
    entity.insert(
        "attrs".to_string(),
        serde_json::json!({
            "active": true
        }),
    );
    entity.insert("parents".to_string(), serde_json::json!([]));

    let entities = vec![Some(entity)];

    // Create a new authorization request
    let request = AzRequestBuilder::new(595307436770, "b1be7dc2fd944c548ac2c66dddd0c61d".to_string())
        .with_request_id(Some("7567".to_string()))
        .with_subject(Some(subject))
        .with_principal(Some(principal))
        .with_entities_map("cedar".to_string(), entities)
        .with_context(Some(context))
        .with_evaluation(evaluation_view)
        .with_evaluation(evaluation_create)
        .build();

    match client.check_auth(Some(request)).await {
        Ok(response) => {
            if response.decision {
                println!("✅ Authorization Permitted");
            } else {
                println!("❌ Authorization Denied");
                if let Some(ctx) = response.context {
                    if let Some(admin) = ctx.reason_admin {
                        println!("-> Reason Admin: {}", admin.message);
                    }
                    if let Some(user) = ctx.reason_user {
                        println!("-> Reason User: {}", user.message);
                    }
                }

                for eval in response.evaluations {
                    if eval.decision {
                        println!("-> ✅ Authorization Permitted");
                    }
                    if let Some(ctx) = eval.context {
                        if let Some(admin) = ctx.reason_admin {
                            println!("-> Reason Admin: {}", admin.message);
                        }
                        if let Some(user) = ctx.reason_user {
                            println!("-> Reason User: {}", user.message);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to check auth: {}", e);
            return Err(Err(e.into()));
        }
    }

    Ok(Ok(()))
}
