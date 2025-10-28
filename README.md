# The official Rust SDK for Permguard

[![GitHub License](https://img.shields.io/github/license/permguard/sdk-rust)](https://github.com/permguard/sdk-rust?tab=Apache-2.0-1-ov-file#readme)
[![X (formerly Twitter) Follow](https://img.shields.io/twitter/follow/permguard)](https://x.com/intent/follow?original_referer=https%3A%2F%2Fdeveloper.x.com%2F&ref_src=twsrc%5Etfw%7Ctwcamp%5Ebuttonembed%7Ctwterm%5Efollow%7Ctwgr%5ETwitterDev&screen_name=Permguard)

[![Documentation](https://img.shields.io/website?label=Docs&url=https%3A%2F%2Fwww.permguard.com%2F)](https://www.permguard.com/)

[![Watch the video on YouTube](https://raw.githubusercontent.com/permguard/permguard-assets/refs/heads/main/video/permguard-thumbnail-preview.png)](https://youtu.be/cH_boKCpLQ8?si=i1fWFHT5kxQQJoYN)

[Watch the video on YouTube](https://youtu.be/cH_boKCpLQ8?si=i1fWFHT5kxQQJoYN)


The Permguard Rust SDK provides a simple and flexible client to perform authorization checks against a Permguard Policy Decision Point (PDP) service using gRPC.
Plase refer to the [Permguard Documentation](https://www.permguard.com/) for more information.

---

## Prerequisites

- **Rust Toolchain 1.89.0**

---

## Installation

Add permguard dependency in your cargo.toml file
```

---

## Usage Example

Below is a sample Rust code demonstrating how to create a Permguard client, build an authorization request using a builder pattern, and process the authorization response:

```rust
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
```

---

## Version Compatibility

Our SDK follows a versioning scheme aligned with the PermGuard server versions to ensure seamless integration. The versioning format is as follows:

**SDK Versioning Format:** `x.y.z`

- **x.y**: Indicates the compatible PermGuard server version.
- **z**: Represents the SDK's patch or minor updates specific to that server version.

**Compatibility Examples:**

- `SDK Version 1.3.0` is compatible with `PermGuard Server 1.3`.
- `SDK Version 1.3.1` includes minor improvements or bug fixes for `PermGuard Server 1.3`.

**Incompatibility Example:**

- `SDK Version 1.3.0` **may not be guaranteed** to be compatible with `PermGuard Server 1.4` due to potential changes introduced in server version `1.4`.

**Important:** Ensure that the major and minor versions (`x.y`) of the SDK match those of your PermGuard server to maintain compatibility.

---

Created by [Nitro Agility](https://www.nitroagility.com/).