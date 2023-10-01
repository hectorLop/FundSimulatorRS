use clap::{arg, command, Parser};

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

struct Investment {
    amount: f64,
    years: i64,
    _annual_contribution: f64,
    annual_profitability: f64,
}

impl Investment {
    fn new(amount: f64, years: i64, _annual_contribution: f64, annual_profitability: f64) -> Self {
        Investment {
            amount,
            years,
            _annual_contribution,
            annual_profitability,
        }
    }

    fn simulate(&self) -> Vec<InvestmentStatus> {
        let mut simulation_results: Vec<InvestmentStatus> = Vec::new();
        let mut status: InvestmentStatus;
        for year in 0..self.years {
            if year == 0 {
                status = InvestmentStatus::new(year, self.amount, self.annual_profitability, 0.2);
            } else {
                let last_year_result = simulation_results
                    .last()
                    .unwrap_or_else(|| panic!("Error in year {}", year));
                status = InvestmentStatus::new(
                    year,
                    last_year_result.gross_profit(),
                    self.annual_profitability,
                    0.2,
                );
            }

            simulation_results.push(status);
        }

        simulation_results
    }

    fn results(&self, investment_status: Vec<InvestmentStatus>) -> String {
        let last_year_result = investment_status
            .last()
            .expect("Error getting the last year status for the total results");
        format!(
            "
        -------------------------------------------------------------
        After {} years, these are the total results of the Investment:
        --------------------------------------------------------------
        Initial amount: {}
        Gross profit: {}
        Net profit: {}
        ",
            self.years,
            self.amount,
            last_year_result.gross_profit() - self.amount,
            last_year_result.net_profit() - self.amount
        )
    }
}

struct InvestmentStatus {
    year: i64,
    initial_amount: f64,
    profitability: f64,
    taxes: f64,
}

impl InvestmentStatus {
    fn new(year: i64, initial_amount: f64, profitability: f64, taxes: f64) -> Self {
        InvestmentStatus {
            year,
            initial_amount,
            profitability,
            taxes,
        }
    }

    fn gross_profit(&self) -> f64 {
        self.initial_amount + (self.initial_amount * self.profitability)
    }

    fn net_profit(&self) -> f64 {
        self.gross_profit() - (self.gross_profit() * self.taxes)
    }

    fn results(&self) -> String {
        format!(
            "
        -----------------
        | YEAR {}
        -----------------
        Initial amount: {}
        Gross profit: {}
        Net profit: {}
        ",
            self.year,
            self.initial_amount,
            self.gross_profit(),
            self.net_profit()
        )
    }
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
