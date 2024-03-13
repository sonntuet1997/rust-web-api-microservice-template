use common::options::{default_log, Log};
use serde::Deserialize;

/// Configuration options for the application.
///
/// This struct represents the configuration options for the application, including server settings,
/// database configuration, endpoint for the exporter, service name, and logging configuration.
#[readonly::make]
#[derive(Deserialize, Debug)]
pub struct Options {
    /// Configuration for the servers.
    pub servers: GrpcServers,
    /// The endpoint for the exporter.
    pub exporter_endpoint: String,
    /// The name of the service.
    pub service_name: String,
    /// Configuration for logging, including log level.
    #[serde(default = "default_log")]
    pub log: Log,
}

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
