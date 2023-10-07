use clap::{arg, command, Parser};

mod investment;
mod types;
use investment::Investment;
use types::PositiveFloat;

#[derive(Parser)]
#[command(about = "Simulate index funds behaviour!")]
struct Args {
    #[arg(short, long, help = "Initial amount for the investing")]
    amount: usize,
    #[arg(short, long, help = "Annual percentage of interest")]
    interest: f64,
    #[arg(short, long, help = "Investment years")]
    years: usize,
    #[arg(short, long, help = "Extra contribution per year")]
    contribution: usize,
}

fn main() {
    let args = Args::parse();
    let investment = Investment::new(
        PositiveFloat::try_from(args.amount as f64).unwrap(),
        args.years,
        PositiveFloat::try_from(args.contribution as f64).unwrap(),
        args.interest,
    );
    let investment_status = investment.simulate();

    for status in investment_status.iter() {
        println!("{}", status.results());
    }

    let results = investment.results(investment_status);
    println!("{}", results);
}
