#[derive(Debug, Clone)]
pub struct AzEndpoint {
    pub schema: String,
    pub port: u16,
    pub host: String,
}

impl AzEndpoint {
    pub fn new(schema: impl Into<String>, port: u16, host: impl Into<String>) -> Self {
        let schema = schema.into();
        let host = host.into();

        assert!(!schema.is_empty(), "schema cannot be empty");
        assert!(!host.is_empty(), "host cannot be empty");

        Self { schema, port, host }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AzConfig {
    pub endpoint: Option<AzEndpoint>,
}

impl AzConfig {
    pub fn new() -> Self {
        Self { endpoint: None }
    }

    pub fn with_endpoint(mut self, endpoint: Option<AzEndpoint>) -> Self {
        self.endpoint = endpoint;
        self
    }
}