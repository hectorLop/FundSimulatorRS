use clap::{arg, command, Parser};

mod investment;
use investment::Investment;

#[derive(Parser)]
#[command(about = "Simulate index funds behaviour!")]
struct Args {
    #[arg(short, long, help = "Initial amount for the investing")]
    amount: usize,
    #[arg(short, long, help = "Annual percentage of profit")]
    profit: f64,
    #[arg(short, long, help = "Investment years")]
    years: i64,
    #[arg(short, long, help = "Years with negative return")]
    negative: i64,
}

fn main() {
    let args = Args::parse();
    let investment = Investment::new(args.amount as f64, args.years, 0.0, args.profit);
    let investment_status = investment.simulate();

    for status in investment_status.iter() {
        println!("{}", status.results());
    }

    let results = investment.results(investment_status);
    println!("{}", results);
}
