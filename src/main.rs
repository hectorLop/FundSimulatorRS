use clap::{arg, command, Parser, ValueEnum};

mod cli;
mod distributions;
mod error;
mod investment;
mod investment_config;
mod server;
mod types;

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
async fn main() {
    let args = Args::parse();
    if args.mode == AppMode::Cli && args.config_file.is_none() {
        eprintln!("Error: `config_file` is required when `mode` is set to `Cli`");
    }

    match args.mode {
        AppMode::Cli => cli::run_cli_simulation(args.config_file.unwrap()),
        AppMode::Server => {
            let server = server::Server::new("0.0.0.0".to_string(), "3000".to_string());
            server.serve().await;
        }
    }
}
