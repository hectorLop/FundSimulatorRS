use clap::{arg, command, Parser};
use config::{Config, File, FileFormat};

mod distributions;
mod error;
mod investment;
mod types;

use investment::{Investment, InvestmentSnapshotResult};
use types::{AnnualContribution, Interest, PositiveFloat};

#[derive(Parser)]
#[command(about = "Simulate index funds behaviour!")]
struct Args {
    #[arg(short, long, help = "Configuration file")]
    config_file: String,
}

#[derive(serde::Deserialize)]
struct Configuration {
    deposit: usize,
    interest_rates: Interest,
    years: usize,
    annual_contributions: AnnualContribution,
}

fn main() {
    let args = Args::parse();
    let config: Configuration = Config::builder()
        .add_source(File::new(&args.config_file, FileFormat::Json))
        .build()
        .expect("Error loading configuration file")
        .try_deserialize()
        .expect("Error deserializing the configuration");

    let investment = Investment::new(
        PositiveFloat::try_from(config.deposit as f64).unwrap(),
        config.years,
        config
            .annual_contributions
            .to_annual_contributions(config.years),
        config.interest_rates.to_interest_rates(config.years),
    );
    let investment_snapshots = investment.simulate().unwrap();
    let investment_results: Vec<InvestmentSnapshotResult> = investment_snapshots
        .iter()
        .map(|snapshot| snapshot.result())
        .collect();
    for (year, result) in investment_results.iter().enumerate() {
        println!(
            "Investment result year {}\n {}",
            year + 1,
            serde_json::to_string(result).unwrap()
        );
    }
    let investment_result = investment::get_investment_result(investment_results).unwrap();
    println!(
        "Investment result\n {}",
        serde_json::to_string(&investment_result).unwrap()
    );
}
