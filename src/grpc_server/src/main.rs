use clap::{Parser, Subcommand};
use grpc_server::{controllers, options};
use opentelemetry::global;

use common::loggers::telemetry::init_telemetry;
use common::options::parse_options;
use controllers::gpt_answer::init_gpt_answer_server;
use options::Options;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.version {
        println!(env!("APP_VERSION"));
        return;
    }

    let options: Options = match parse_options(args.config_path) {
        Ok(options) => options,
        Err(err) => {
            eprintln!("Failed to load config: {}", err);
            return;
        }
    };

    if let Some(Commands::Config) = args.command {
        println!("{:#?}", options);
        return;
    }

    init_telemetry(
        options.service_name.as_str(),
        options.exporter_endpoint.as_str(),
        options.log.level.as_str(),
    );

    let gpt_answer_server = tokio::spawn(init_gpt_answer_server(options));

    tokio::try_join!(gpt_answer_server).expect("Failed to run servers");

    global::shutdown_tracer_provider();
}

/// Simple REST server.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Config file
    #[arg(short, long, default_value = "config/00-default.toml")]
    config_path: Vec<String>,
    /// Print version
    #[clap(short, long)]
    version: bool,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    /// Print config
    Config,
}
