use clap::{arg, command, Parser};
use config::{Config, File, FileFormat};
use serde::Deserialize;

mod investment;
mod types;
use investment::Investment;
use types::PositiveFloat;

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

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Interest {
    Single(f64),
    Multiple(Vec<f64>),
}

impl Interest {
    fn to_interest_rates(&self, times: usize) -> Vec<f64> {
        match self {
            Interest::Single(fixed_interest) => {
                (0..times).map(|_| *fixed_interest).collect::<Vec<f64>>()
            }
            Interest::Multiple(multiple) => multiple.to_vec(),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum AnnualContribution {
    Single(PositiveFloat),
    Multiple(Vec<PositiveFloat>),
}

impl AnnualContribution {
    fn to_annual_contributions(&self, times: usize) -> Vec<PositiveFloat> {
        match self {
            AnnualContribution::Single(fixed_contribution) => {
                (0..times).map(|_| *fixed_contribution).collect()
            }
            AnnualContribution::Multiple(multiple) => multiple.to_vec(),
        }
    }
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
        config.annual_contributions,
        config.interest_rates,
    );
    let investment_status = investment.simulate();

    for status in investment_status.iter() {
        println!("{}", status.results());
    }

    let results = investment.results(investment_status);
    println!("{}", results);
}
