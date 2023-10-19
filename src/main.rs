use clap::{arg, command, Parser};
use config::{Config, File, FileFormat};
use serde::Deserialize;

mod distributions;
mod investment;
mod types;

use investment::Investment;
use types::{AnnualContribution, Interest, PositiveFloat};

#[derive(Parser)]
#[command(about = "Simulate index funds behaviour!")]
struct Args {
    #[arg(short, long, help = "Configuration file")]
    config_file: String,
}

#[derive(Deserialize)]
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
    let investment_status = investment.simulate();

    for status in investment_status.iter() {
        println!("{}", status.results());
    }

    let results = investment.results(investment_status);
    println!("{}", results);
}
