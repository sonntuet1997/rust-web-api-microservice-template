use serde::Deserialize;

/// Represents servers configuration options.
#[derive(Deserialize, Debug)]
pub struct GrpcServers {
    /// Configuration for using in-memory database.
    pub gpt_answer_service: Option<ServiceServer>,
}

/// Represents service server configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct ServiceServer {
    /// URL for the server.
    pub url: String,
}
