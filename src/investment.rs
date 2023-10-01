pub struct Investment {
    amount: f64,
    years: i64,
    _annual_contribution: f64,
    annual_profitability: f64,
}

impl Investment {
    pub fn new(
        amount: f64,
        years: i64,
        _annual_contribution: f64,
        annual_profitability: f64,
    ) -> Self {
        Investment {
            amount,
            years,
            _annual_contribution,
            annual_profitability,
        }
    }

    pub fn simulate(&self) -> Vec<InvestmentStatus> {
        let mut simulation_results: Vec<InvestmentStatus> = Vec::new();
        let mut status: InvestmentStatus;
        for year in 0..self.years {
            if year == 0 {
                status = InvestmentStatus::new(
                    year,
                    self.amount,
                    self.amount,
                    self.annual_profitability,
                    0.2,
                );
            } else {
                let last_year_result = simulation_results
                    .last()
                    .unwrap_or_else(|| panic!("Error in year {}", year));
                status = InvestmentStatus::new(
                    year,
                    self.amount,
                    last_year_result.gross_profit(),
                    self.annual_profitability,
                    0.2,
                );
            }

            simulation_results.push(status);
        }

        simulation_results
    }

    pub fn results(&self, investment_status: Vec<InvestmentStatus>) -> String {
        let last_year_result = investment_status
            .last()
            .expect("Error getting the last year status for the total results");
        format!(
            "
        -------------------------------------------------------------
        After {} years, these are the total results of the Investment:
        --------------------------------------------------------------
        Initial amount: {}
        Interest gross profit: {}
        Interest net profit: {}
        ",
            self.years,
            self.amount,
            last_year_result.gross_profit() - self.amount,
            last_year_result.net_profit() - self.amount
        )
    }
}

pub struct InvestmentStatus {
    year: i64,
    initial_investment_amount: f64,
    amount: f64,
    profitability: f64,
    taxes: f64,
}

impl InvestmentStatus {
    fn new(
        year: i64,
        initial_investment_amount: f64,
        amount: f64,
        profitability: f64,
        taxes: f64,
    ) -> Self {
        InvestmentStatus {
            year,
            initial_investment_amount,
            amount,
            profitability,
            taxes,
        }
    }

    pub fn interest(&self) -> f64 {
        self.amount * self.profitability
    }

    pub fn gross_profit(&self) -> f64 {
        self.amount + self.interest()
    }

    fn net_profit(&self) -> f64 {
        if self.gross_profit() < self.initial_investment_amount {
            return self.gross_profit();
        }
        let profit = self.gross_profit() - self.initial_investment_amount;
        self.gross_profit() - (profit * self.taxes)
    }

    pub fn results(&self) -> String {
        format!(
            "
        -----------------
        | YEAR {}
        -----------------
        Total contributions: {}
        Interest: {}
        Gross profit: {}
        Net profit: {}
        ",
            self.year,
            self.amount,
            self.interest(),
            self.gross_profit(),
            self.net_profit()
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Investment, InvestmentStatus};
    use assert_float_eq::{afe_is_f64_near, afe_near_error_msg, assert_f64_near};

    #[test]
    fn test_positive_profit_computation() {
        let status = InvestmentStatus::new(1, 10000.0, 10000.0, 0.05, 0.2);
        assert_f64_near!(status.interest(), 500.0);
        assert_f64_near!(status.gross_profit(), 10500.0);
        assert_f64_near!(status.net_profit(), 10400.0);
    }

    #[test]
    fn test_negative_profit_computation() {
        let status = InvestmentStatus::new(1, 10000.0, 10000.0, -0.05, 0.2);
        assert_f64_near!(status.interest(), -500.0);
        assert_f64_near!(status.gross_profit(), 9500.0);
        assert_f64_near!(status.net_profit(), 9500.0);
    }

    #[test]
    fn test_profit_computation() {
        let status = InvestmentStatus::new(0, 10000.0, 10000.0, 0.05, 0.2);
        assert_f64_near!(status.interest(), 500.0);
        assert_f64_near!(status.gross_profit(), 10500.0);
        assert_f64_near!(status.net_profit(), 10400.0);

        let status_second_year = InvestmentStatus::new(1, 10000.0, 10500.0, 0.05, 0.2);
        assert_f64_near!(status_second_year.interest(), 525.0);
        assert_f64_near!(status_second_year.gross_profit(), 11025.0);
        assert_f64_near!(status_second_year.net_profit(), 10820.0);

        let status_third_year = InvestmentStatus::new(2, 10000.0, 11025.0, 0.01, 0.2);
        assert_f64_near!(status_third_year.interest(), 110.25);
        assert_f64_near!(status_third_year.gross_profit(), 11135.25);
        assert_f64_near!(status_third_year.net_profit(), 10908.2);

        let status_fourth_year = InvestmentStatus::new(3, 10000.0, 11135.25, -0.05, 0.2);
        assert_f64_near!(status_fourth_year.interest(), -556.7625);
        assert_f64_near!(status_fourth_year.gross_profit(), 10578.4875);
        assert_f64_near!(status_fourth_year.net_profit(), 10462.79);
    }

    #[test]
    fn test_investment_simulation() {
        let investment = Investment::new(10000.0, 3, 0.0, 0.05);
        let investment_results = investment.simulate();
        let expected: [InvestmentStatus; 3] = [
            InvestmentStatus::new(0, 10000.0, 10000.0, 0.05, 0.2),
            InvestmentStatus::new(0, 10000.0, 10500.0, 0.05, 0.2),
            InvestmentStatus::new(0, 10000.0, 11025.0, 0.05, 0.2),
        ];

        for (i, result) in investment_results.iter().enumerate() {
            assert_f64_near!(result.interest(), expected[i].interest());
            assert_f64_near!(result.gross_profit(), expected[i].gross_profit());
            assert_f64_near!(result.net_profit(), expected[i].net_profit());
        }
    }
}
