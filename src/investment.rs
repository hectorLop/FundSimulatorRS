use crate::types::PositiveFloat;
use fake::Dummy;

#[derive(Debug, Clone, Dummy)]
pub struct Investment {
    deposit: PositiveFloat,
    years: usize,
    annual_contributions: Vec<PositiveFloat>,
    interest_rates: Vec<f64>,
}

impl Investment {
    pub fn new(
        deposit: PositiveFloat,
        years: usize,
        contributions: Vec<PositiveFloat>,
        interest: Vec<f64>,
    ) -> Self {
        Investment {
            deposit,
            years,
            annual_contributions: contributions,
            interest_rates: interest,
        }
    }

    pub fn simulate(&self) -> Vec<InvestmentStatus> {
        let mut simulation_results: Vec<InvestmentStatus> = Vec::new();
        let taxes: PositiveFloat = PositiveFloat(0.2);

        for (i, year) in (0..self.years).enumerate() {
            if year == 0 {
                simulation_results.push(InvestmentStatus::new(
                    year,
                    (self.deposit.0 + self.annual_contributions[i].0)
                        .try_into()
                        .expect("The deposited money cannot be negative"),
                    self.deposit.0 + self.annual_contributions[i].0,
                    self.interest_rates[i],
                    taxes,
                ));
            } else {
                let last_year_result = simulation_results
                    .last()
                    .unwrap_or_else(|| panic!("Error in year {}", year));
                simulation_results.push(InvestmentStatus::new(
                    year,
                    (last_year_result.deposited.0 + self.annual_contributions[i].0)
                        .try_into()
                        .expect("The deposited money cannot be negative"),
                    last_year_result.gross_profit() + self.annual_contributions[i].0,
                    self.interest_rates[i],
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
mod investment_status_tests {
    use super::InvestmentStatus;
    use crate::types::PositiveFloat;
    use assert_float_eq::{afe_is_f64_near, afe_near_error_msg, assert_f64_near};
    use fake::{Fake, Faker};
    use rand::Rng;
    use rstest::rstest;

    #[rstest(
    initial_balance, interest_rate, expected_interest, expected_gross_profit, expected_net_profit,
    case::positive_profit(10000.0, 0.05, 500.0, 10500.0, 10400.0),
    case::negative_profit(10000.0, -0.05, -500.0, 9500.0, 9500.0),
    case::zero_initial_balance(0.0, 0.05, 0.0, 0.0, 0.0),
    case::zero_interest_rate(1000.0, 0.0, 0.0, 1000.0, 1000.0),
    )]
    fn test_profit_computation(
        initial_balance: f64,
        interest_rate: f64,
        expected_interest: f64,
        expected_gross_profit: f64,
        expected_net_profit: f64,
    ) {
        let status = InvestmentStatus::new(
            1,
            PositiveFloat::try_from(initial_balance).unwrap(),
            initial_balance,
            interest_rate,
            PositiveFloat::try_from(0.2).unwrap(),
        );
        assert_f64_near!(status.interest(), expected_interest);
        assert_f64_near!(status.gross_profit(), expected_gross_profit);
        assert_f64_near!(status.net_profit(), expected_net_profit);
    }

    #[test]
    fn test_multi_year_profit_computation() {
        let balances: [f64; 4] = [10000.0, 10500.0, 11025.0, 11135.25];
        let interest_rates: [f64; 4] = [0.05, 0.05, 0.01, -0.05];
        let interests: [f64; 4] = [500.0, 525.0, 110.25, -556.7625];
        let gross_profits: [f64; 4] = [10500.0, 11025.0, 11135.25, 10578.4875];
        let net_profits: [f64; 4] = [10400.0, 10820.0, 10908.2, 10462.79];

        for year in 0..4 {
            let status = InvestmentStatus::new(
                year,
                PositiveFloat::try_from(10000.0).unwrap(),
                balances[year],
                interest_rates[year],
                PositiveFloat::try_from(0.2).unwrap(),
            );
            assert_f64_near!(status.interest(), interests[year]);
            assert_f64_near!(status.gross_profit(), gross_profits[year]);
            assert_f64_near!(status.net_profit(), net_profits[year]);
        }
    }

    #[derive(Clone, Debug)]
    struct InterestRateFixture(pub f64);

    impl quickcheck::Arbitrary for InterestRateFixture {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let mut rng = rand::thread_rng();
            Self(rng.gen_range(-2.0..2.0))
        }
    }
    impl quickcheck::Arbitrary for PositiveFloat {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            Faker.fake()
        }
    }

    // Non-negativity: interest, gross profit, and net profit should never be negative.
    #[quickcheck_macros::quickcheck]
    fn non_negativity(balance: PositiveFloat, interest_rate: InterestRateFixture) -> bool {
        let status = InvestmentStatus::new(
            0,
            balance,
            balance.0,
            interest_rate.0.abs(),
            PositiveFloat::try_from(0.2).unwrap(),
        );
        status.interest() >= 0.0 && status.gross_profit() >= 0.0 && status.net_profit() >= 0.0
    }

    // Monotonicity: Increasing the balance or interest rate on positive years should not decrease any of the computed values.
    #[quickcheck_macros::quickcheck]
    fn monotonicity(
        initial_balance: PositiveFloat,
        balance: PositiveFloat,
        interest_rate: InterestRateFixture,
    ) -> bool {
        let status1 = InvestmentStatus::new(
            0,
            initial_balance,
            balance.0,
            interest_rate.0.abs(),
            PositiveFloat::try_from(0.2).unwrap(),
        );
        let increased_balance = PositiveFloat::try_from(balance.0 * 1.1).unwrap();
        let increased_interest_rate = interest_rate.0.abs();

        let status2 = InvestmentStatus::new(
            0,
            initial_balance,
            increased_balance.0,
            increased_interest_rate,
            PositiveFloat::try_from(0.2).unwrap(),
        );
        status2.interest() >= status1.interest()
            && status2.gross_profit() >= status1.gross_profit()
            && status2.net_profit() >= status1.net_profit()
    }
}

#[cfg(test)]
mod test {
    use super::{Investment, InvestmentStatus, PositiveFloat};
    use crate::{AnnualContribution, Interest};
    use assert_float_eq::{afe_is_f64_near, afe_near_error_msg, assert_f64_near};

    #[test]
    fn test_investment_simulation() {
        let investment = Investment::new(
            PositiveFloat::try_from(10000.0).unwrap(),
            3,
            AnnualContribution::Single(PositiveFloat(0.0)).to_annual_contributions(3),
            Interest::Single(0.05).to_interest_rates(3),
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
            AnnualContribution::Single(PositiveFloat(3600.0)).to_annual_contributions(3),
            Interest::Single(0.05).to_interest_rates(3),
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
            AnnualContribution::Single(PositiveFloat(3600.0)).to_annual_contributions(3),
            Interest::Single(-0.05).to_interest_rates(3),
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
