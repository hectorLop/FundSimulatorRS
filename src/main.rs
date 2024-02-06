use clap::{arg, command, Parser, ValueEnum};

use fund_simulator_rs::error;
use fund_simulator_rs::cli;
use fund_simulator_rs::server;
use fund_simulator_rs::configuration;

#[derive(Clone, ValueEnum, Debug, PartialEq)]
enum AppMode {
    Server,
    Cli,
}

#[derive(Parser, Debug)]
#[command(about = "Simulate index funds behaviour!")]
struct Args {
    #[arg(short, long, help = "Application mode")]
    mode: AppMode,
    #[arg(short, long, help = "Configuration file", required = false)]
    config_file: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), error::ApplicationError> {
    let args = Args::parse();
    if args.mode == AppMode::Cli && args.config_file.is_none() {
        eprintln!("Error: `config_file` is required when `mode` is set to `Cli`");
    }

    match args.mode {
        AppMode::Cli => cli::run_cli_simulation(args.config_file.unwrap()),
        AppMode::Server => {
            let configuration = configuration::Configuration::load()?;
            let pool = sqlx::PgPool::connect(&configuration.get_postgres_url()).await?;
            let server =
                server::Server::new("0.0.0.0".to_string(), configuration.application_port, &pool);
            server.serve().await?;
        }
    }

    Ok(())
}
