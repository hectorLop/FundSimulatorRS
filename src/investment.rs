use crate::types::PositiveFloat;
use fake::Dummy;

#[derive(Debug, Clone, Dummy)]
pub struct Investment {
    initial_deposit: PositiveFloat,
    investment_years: usize,
    annual_net_contributions: Vec<PositiveFloat>,
    return_rates: Vec<f64>,
}

impl Investment {
    pub fn new(
        initial_deposit: PositiveFloat,
        investment_years: usize,
        annual_net_contributions: Vec<PositiveFloat>,
        return_rates: Vec<f64>,
    ) -> Self {
        Investment {
            initial_deposit,
            investment_years,
            annual_net_contributions,
            return_rates,
        }
    }

    pub fn simulate(&self) -> Vec<InvestmentSnapshot> {
        let mut simulation_results: Vec<InvestmentSnapshot> = Vec::new();

        for (i, year) in (0..self.investment_years).enumerate() {
            let net_contribution: PositiveFloat = {
                if year == 0 {
                    self.initial_deposit.0 + self.annual_net_contributions[i].0
                } else {
                    simulation_results[i - 1].net_contribution.0
                        + self.annual_net_contributions[i].0
                }
            }
            .try_into()
            .expect("Net contribution canot be negative");
            let initial_balance = {
                if year == 0 {
                    self.initial_deposit.0 + self.annual_net_contributions[i].0
                } else {
                    simulation_results[i - 1].final_balance() + self.annual_net_contributions[i].0
                }
            };

            let investment_snapshot = InvestmentSnapshot::new(
                year,
                net_contribution,
                initial_balance,
                self.return_rates[i],
            )
            .expect("NaN value found");
            simulation_results.push(investment_snapshot);
        }

        simulation_results
    }

    pub fn results(&self, investment_status: Vec<InvestmentSnapshot>) -> String {
        let last_year_result = investment_status
            .last()
            .expect("Error getting the last year status for the total results");
        format!(
            "
        -------------------------------------------------------------
        After {} years, these are the total results of the Investment:
        --------------------------------------------------------------
        Net contributions: {}
        Final balance: {} 
        ",
            self.investment_years,
            last_year_result.net_contribution.0,
            last_year_result.final_balance()
        )
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct InvestmentSnapshot {
    year: usize,
    net_contribution: PositiveFloat,
    initial_balance: f64,
    return_rate: f64,
}

impl InvestmentSnapshot {
    fn new(
        year: usize,
        net_contribution: PositiveFloat,
        initial_balance: f64,
        return_rate: f64,
    ) -> Result<Self, &'static str> {
        if initial_balance.is_nan() || return_rate.is_nan() {
            return Err("NaN value provided");
        }
        Ok(InvestmentSnapshot {
            year,
            net_contribution,
            initial_balance,
            return_rate,
        })
    }

    pub fn result(&self) -> InvestmentSnapshotResult {
        InvestmentSnapshotResult {
            year: self.year,
            net_contribution: self.net_contribution,
            initial_balance: self.initial_balance,
            return_rate: self.return_rate,
            final_balance: self.final_balance(),
        }
    }

    fn final_balance(&self) -> f64 {
        self.initial_balance + (self.initial_balance * self.return_rate)
    }
}

pub struct InvestmentSnapshotResult {
    year: usize,
    net_contribution: PositiveFloat,
    initial_balance: f64,
    return_rate: f64,
    final_balance: f64,
}

impl InvestmentSnapshotResult {
    pub fn report(&self) -> String {
        format!(
            "
        -----------------
        | YEAR {}
        -----------------
        Net contribution: {}
        Initial balance: {}
        Return rate: {}
        Final balance: {}
        ",
            self.year,
            self.net_contribution.0,
            self.initial_balance,
            self.return_rate,
            self.final_balance
        )
    }
}

#[cfg(test)]
mod investment_status_tests {
    use super::InvestmentSnapshot;
    use crate::types::PositiveFloat;
    use fake::{Fake, Faker};
    use rand::Rng;

    #[derive(Clone, Debug)]
    struct ReturnRateFixture(pub f64);

    impl quickcheck::Arbitrary for ReturnRateFixture {
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

    #[derive(Clone, Debug)]
    struct FloatFixture(pub f64);

    impl quickcheck::Arbitrary for FloatFixture {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let mut rng = rand::thread_rng();
            Self(rng.gen_range(-100000.0..100000.0))
        }
    }

    #[quickcheck_macros::quickcheck]
    fn final_balance_properties(balance: PositiveFloat, return_rate: ReturnRateFixture) -> bool {
        let status = InvestmentSnapshot::new(0, balance, balance.0, return_rate.0).unwrap();
        let result = status.result();

        if return_rate.0 == 0.0 {
            return result.final_balance == status.initial_balance;
        } else if return_rate.0 < 0.0 {
            return result.final_balance < status.initial_balance;
        } else {
            return result.final_balance > status.initial_balance;
        }
    }

    #[quickcheck_macros::quickcheck]
    fn investment_snapshot_result_consistency(
        year: usize,
        net_contribution: PositiveFloat,
        initial_balance: FloatFixture,
        return_rate: FloatFixture,
    ) -> bool {
        let status =
            InvestmentSnapshot::new(year, net_contribution, initial_balance.0, return_rate.0)
                .unwrap();
        let result = status.result();

        result.year == status.year
            && result.net_contribution == status.net_contribution
            && result.initial_balance == status.initial_balance
            && result.return_rate == status.return_rate
    }

    #[test]
    fn test_investment_snapshot_with_nan() {
        let status = InvestmentSnapshot::new(2022, PositiveFloat(1000.0), std::f64::NAN, 0.12);
        assert!(status.is_err());
        let status = InvestmentSnapshot::new(2022, PositiveFloat(1000.0), 10000.0, std::f64::NAN);
        assert!(status.is_err());
    }
}

#[cfg(test)]
mod test_investment {
    use super::{Investment, PositiveFloat};
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
        let expected: [f64; 3] = [10500.0, 11025.0, 11576.25];

        for (i, result) in investment_results.iter().enumerate() {
            assert_f64_near!(result.final_balance(), expected[i]);
        }
    }

    #[test]
    fn test_investment_simulation_with_annual_contribution() {
        let investment = Investment::new(
            PositiveFloat::try_from(10000.0).unwrap(),
            3,
            AnnualContribution::Single(PositiveFloat(3600.0)).to_annual_contributions(3),
            vec![0.05, 0.05, 0.05],
        );
        let investment_results = investment.simulate();
        let expected: [f64; 3] = [14280.0, 18774.0, 23492.7];

        for (i, result) in investment_results.iter().enumerate() {
            assert_f64_near!(result.final_balance(), expected[i]);
        }
    }

    #[test]
    fn test_investment_simulation_with_annual_contribution_and_negative_rates() {
        let investment = Investment::new(
            PositiveFloat::try_from(10000.0).unwrap(),
            3,
            AnnualContribution::Single(PositiveFloat(3600.0)).to_annual_contributions(3),
            vec![0.05, -0.05, -0.05],
        );
        let investment_results = investment.simulate();
        let expected: [f64; 3] = [14280.0, 16986.0, 19556.7];

        for (i, result) in investment_results.iter().enumerate() {
            assert_f64_near!(result.final_balance(), expected[i]);
        }
    }
}
