use crate::types::PositiveFloat;
use fake::Dummy;

#[derive(Debug, Clone, Copy, Dummy)]
pub struct Investment {
    deposit: PositiveFloat,
    #[dummy(faker = "1..1000")]
    years: usize,
    annual_contribution: PositiveFloat,
    interest_rate: f64,
}

impl Investment {
    pub fn new(
        deposit: PositiveFloat,
        years: usize,
        annual_contribution: PositiveFloat,
        interest_rate: f64,
    ) -> Self {
        Investment {
            deposit,
            years,
            annual_contribution,
            interest_rate,
        }
    }

    pub fn simulate(&self) -> Vec<InvestmentStatus> {
        let mut simulation_results: Vec<InvestmentStatus> = Vec::new();
        let taxes: PositiveFloat = PositiveFloat(0.2);
        for year in 0..self.years {
            if year == 0 {
                simulation_results.push(InvestmentStatus::new(
                    year,
                    (self.deposit.0 + self.annual_contribution.0)
                        .try_into()
                        .expect("The deposited money cannot be negative"),
                    self.deposit.0 + self.annual_contribution.0,
                    self.interest_rate,
                    taxes,
                ));
            } else {
                let last_year_result = simulation_results
                    .last()
                    .unwrap_or_else(|| panic!("Error in year {}", year));
                simulation_results.push(InvestmentStatus::new(
                    year,
                    (last_year_result.deposited.0 + self.annual_contribution.0)
                        .try_into()
                        .expect("The deposited money cannot be negative"),
                    last_year_result.gross_profit() + self.annual_contribution.0,
                    self.interest_rate,
                    taxes,
                ));
            }
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
        Total deposited: {}
        Interest gross profit: {}
        Interest net profit: {}
        ",
            self.years,
            last_year_result.deposited.0,
            last_year_result.gross_profit() - last_year_result.deposited.0,
            last_year_result.net_profit() - last_year_result.deposited.0
        )
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct InvestmentStatus {
    year: usize,
    deposited: PositiveFloat,
    balance: f64,
    interest_rate: f64,
    taxes: PositiveFloat,
}

impl InvestmentStatus {
    fn new(
        year: usize,
        deposited: PositiveFloat,
        balance: f64,
        interest_rate: f64,
        taxes: PositiveFloat,
    ) -> Self {
        InvestmentStatus {
            year,
            deposited,
            balance,
            interest_rate,
            taxes,
        }
    }

    pub fn interest(&self) -> f64 {
        self.balance * self.interest_rate
    }

    pub fn gross_profit(&self) -> f64 {
        self.balance + self.interest()
    }

    fn net_profit(&self) -> f64 {
        if self.gross_profit() < self.deposited.0 {
            return self.gross_profit();
        }
        let profit = self.gross_profit() - self.deposited.0;
        self.gross_profit() - (profit * self.taxes.0)
    }

    pub fn results(&self) -> String {
        format!(
            "
        -----------------
        | YEAR {}
        -----------------
        Total deposited: {}
        Interest: {}
        Gross balance: {}
        Net balance: {}
        ",
            self.year,
            self.deposited.0,
            self.interest(),
            self.gross_profit(),
            self.net_profit()
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Investment, InvestmentStatus, PositiveFloat};
    use assert_float_eq::{afe_is_f64_near, afe_near_error_msg, assert_f64_near};
    use fake::{Fake, Faker};

    impl quickcheck::Arbitrary for InvestmentStatus {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            Faker.fake()
        }
    }

    impl quickcheck::Arbitrary for Investment {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            Faker.fake()
        }
    }

    #[quickcheck_macros::quickcheck]
    fn test_investment_simulate(investment: Investment) -> bool {
        let results = investment.simulate();

        for i in 0..results.len() {
            if i != 0 {
                if investment.interest_rate > 0.0 {
                    let conditions: [bool; 3] = [
                        results[i].interest() > results[i - 1].interest(),
                        results[i].gross_profit() > results[i - 1].gross_profit(),
                        results[i].net_profit() > results[i - 1].net_profit(),
                    ];

                    if !conditions.iter().all(|&x| x == true) {
                        return false;
                    }
                } else {
                    let conditions: [bool; 3] = [
                        results[i].interest() < results[i - 1].interest(),
                        results[i].gross_profit() < results[i - 1].gross_profit(),
                        results[i].net_profit() < results[i - 1].net_profit(),
                    ];

                    if !conditions.iter().all(|&x| x == true) {
                        return false;
                    }
                }
            }
        }
        true
    }

    #[quickcheck_macros::quickcheck]
    fn test_investment_status_interest(status: InvestmentStatus) -> bool {
        status.interest() < status.balance
    }

    #[quickcheck_macros::quickcheck]
    fn test_investment_status_gross_profit(status: InvestmentStatus) -> bool {
        status.gross_profit() > status.interest()
    }
    #[quickcheck_macros::quickcheck]
    fn test_investment_status_net_profit(status: InvestmentStatus) -> bool {
        status.net_profit() <= status.gross_profit()
    }

    #[test]
    fn test_positive_profit_computation() {
        let status = InvestmentStatus::new(
            1,
            PositiveFloat::try_from(10000.0).unwrap(),
            10000.0,
            0.05,
            PositiveFloat::try_from(0.2).unwrap(),
        );
        assert_f64_near!(status.interest(), 500.0);
        assert_f64_near!(status.gross_profit(), 10500.0);
        assert_f64_near!(status.net_profit(), 10400.0);
    }

    #[test]
    fn test_negative_profit_computation() {
        let status = InvestmentStatus::new(
            1,
            PositiveFloat::try_from(10000.0).unwrap(),
            10000.0,
            -0.05,
            PositiveFloat::try_from(0.2).unwrap(),
        );
        assert_f64_near!(status.interest(), -500.0);
        assert_f64_near!(status.gross_profit(), 9500.0);
        assert_f64_near!(status.net_profit(), 9500.0);
    }

    #[test]
    fn test_profit_computation() {
        let status = InvestmentStatus::new(
            0,
            PositiveFloat::try_from(10000.0).unwrap(),
            10000.0,
            0.05,
            PositiveFloat::try_from(0.2).unwrap(),
        );
        assert_f64_near!(status.interest(), 500.0);
        assert_f64_near!(status.gross_profit(), 10500.0);
        assert_f64_near!(status.net_profit(), 10400.0);

        let status_second_year = InvestmentStatus::new(
            1,
            PositiveFloat::try_from(10000.0).unwrap(),
            10500.0,
            0.05,
            PositiveFloat::try_from(0.2).unwrap(),
        );
        assert_f64_near!(status_second_year.interest(), 525.0);
        assert_f64_near!(status_second_year.gross_profit(), 11025.0);
        assert_f64_near!(status_second_year.net_profit(), 10820.0);

        let status_third_year = InvestmentStatus::new(
            2,
            PositiveFloat::try_from(10000.0).unwrap(),
            11025.0,
            0.01,
            PositiveFloat::try_from(0.2).unwrap(),
        );
        assert_f64_near!(status_third_year.interest(), 110.25);
        assert_f64_near!(status_third_year.gross_profit(), 11135.25);
        assert_f64_near!(status_third_year.net_profit(), 10908.2);

        let status_fourth_year = InvestmentStatus::new(
            3,
            PositiveFloat::try_from(10000.0).unwrap(),
            11135.25,
            -0.05,
            PositiveFloat::try_from(0.2).unwrap(),
        );
        assert_f64_near!(status_fourth_year.interest(), -556.7625);
        assert_f64_near!(status_fourth_year.gross_profit(), 10578.4875);
        assert_f64_near!(status_fourth_year.net_profit(), 10462.79);
    }

    #[test]
    fn test_investment_simulation() {
        let investment = Investment::new(
            PositiveFloat::try_from(10000.0).unwrap(),
            3,
            PositiveFloat::try_from(0.0).unwrap(),
            0.05,
        );
        let investment_results = investment.simulate();
        let expected: [InvestmentStatus; 3] = [
            InvestmentStatus::new(
                0,
                PositiveFloat::try_from(10000.0).unwrap(),
                10000.0,
                0.05,
                PositiveFloat::try_from(0.2).unwrap(),
            ),
            InvestmentStatus::new(
                0,
                PositiveFloat::try_from(10000.0).unwrap(),
                10500.0,
                0.05,
                PositiveFloat::try_from(0.2).unwrap(),
            ),
            InvestmentStatus::new(
                0,
                PositiveFloat::try_from(10000.0).unwrap(),
                11025.0,
                0.05,
                PositiveFloat::try_from(0.2).unwrap(),
            ),
        ];

        for (i, result) in investment_results.iter().enumerate() {
            assert_f64_near!(result.interest(), expected[i].interest());
            assert_f64_near!(result.gross_profit(), expected[i].gross_profit());
            assert_f64_near!(result.net_profit(), expected[i].net_profit());
        }
    }

    #[test]
    fn test_investment_simulation_with_annual_contribution() {
        let investment = Investment::new(
            PositiveFloat::try_from(10000.0).unwrap(),
            3,
            PositiveFloat::try_from(3600.0).unwrap(),
            0.05,
        );
        let investment_results = investment.simulate();
        let expected: [InvestmentStatus; 3] = [
            InvestmentStatus::new(
                0,
                PositiveFloat::try_from(13600.0).unwrap(),
                13600.0,
                0.05,
                PositiveFloat::try_from(0.2).unwrap(),
            ),
            InvestmentStatus::new(
                0,
                PositiveFloat::try_from(17200.0).unwrap(),
                17880.0,
                0.05,
                PositiveFloat::try_from(0.2).unwrap(),
            ),
            InvestmentStatus::new(
                0,
                PositiveFloat::try_from(20800.0).unwrap(),
                22374.0,
                0.05,
                PositiveFloat::try_from(0.2).unwrap(),
            ),
        ];

        for (i, result) in investment_results.iter().enumerate() {
            assert_f64_near!(result.interest(), expected[i].interest());
            assert_f64_near!(result.gross_profit(), expected[i].gross_profit());
            assert_f64_near!(result.net_profit(), expected[i].net_profit());
        }
    }

    #[test]
    fn test_investment_simulation_with_annual_contribution_and_negative_rates() {
        let investment = Investment::new(
            PositiveFloat::try_from(10000.0).unwrap(),
            3,
            PositiveFloat::try_from(3600.0).unwrap(),
            -0.05,
        );
        let investment_results = investment.simulate();
        let expected: [InvestmentStatus; 3] = [
            InvestmentStatus::new(
                0,
                PositiveFloat::try_from(13600.0).unwrap(),
                13600.0,
                -0.05,
                PositiveFloat::try_from(0.2).unwrap(),
            ),
            InvestmentStatus::new(
                0,
                PositiveFloat::try_from(17200.0).unwrap(),
                16520.0,
                -0.05,
                PositiveFloat::try_from(0.2).unwrap(),
            ),
            InvestmentStatus::new(
                0,
                PositiveFloat::try_from(20800.0).unwrap(),
                19294.0,
                -0.05,
                PositiveFloat::try_from(0.2).unwrap(),
            ),
        ];

        for (i, result) in investment_results.iter().enumerate() {
            assert_f64_near!(result.interest(), expected[i].interest());
            assert_f64_near!(result.gross_profit(), expected[i].gross_profit());
            assert_f64_near!(result.net_profit(), expected[i].net_profit());
        }
    }
}
