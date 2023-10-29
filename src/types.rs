use crate::distributions::get_distributions;
use fake::{Dummy, Faker};
use rand::prelude::SliceRandom;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct PositiveFloat(pub f64);

#[derive(Debug, PartialEq)]
pub struct PositiveFloatError(String);

impl TryFrom<f64> for PositiveFloat {
    type Error = PositiveFloatError;

    fn try_from(value: f64) -> Result<Self, PositiveFloatError> {
        if value < 0.0 {
            return Err(PositiveFloatError(format!(
                "{:.2} is a negative float.",
                value
            )));
        }

        Ok(Self(value))
    }
}

impl Dummy<Faker> for PositiveFloat {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Self(rng.gen_range(0.0..10000000.0))
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Interest {
    Single(f64),
    Multiple(Vec<f64>),
    Distribution(String),
}

impl Interest {
    pub fn to_interest_rates(&self, times: usize) -> Vec<f64> {
        match self {
            Interest::Single(fixed_interest) => {
                (0..times).map(|_| *fixed_interest).collect::<Vec<f64>>()
            }
            Interest::Multiple(multiple) => multiple.to_vec(),
            Interest::Distribution(dist_name) => {
                let distributions = get_distributions();
                let selected_distribution = distributions
                    .get(dist_name.as_str())
                    .expect("The selected distribution doesn't exist");

                let mut rng = rand::thread_rng();
                selected_distribution
                    .choose_multiple(&mut rng, times)
                    .copied()
                    .collect()
            }
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
    pub fn to_annual_contributions(&self, times: usize) -> Vec<PositiveFloat> {
        match self {
            AnnualContribution::Single(fixed_contribution) => {
                (0..times).map(|_| *fixed_contribution).collect()
            }
            AnnualContribution::Multiple(multiple) => multiple.to_vec(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Interest, PositiveFloat};
    use claim::assert_ok_eq;
    use rand::Rng;

    #[derive(Clone, Debug)]
    struct ValidNumberFixture(pub f64);

    #[derive(Clone, Debug)]
    struct InvalidNumberFixture(pub f64);

    impl quickcheck::Arbitrary for ValidNumberFixture {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let mut rng = rand::thread_rng();
            Self(rng.gen_range(0.0..10000000000.0))
        }
    }

    impl quickcheck::Arbitrary for InvalidNumberFixture {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let mut rng = rand::thread_rng();
            Self(rng.gen_range(-10000000000.0..-0.000001))
        }
    }

    #[quickcheck_macros::quickcheck]
    fn test_positive_float_creation(valid_number: ValidNumberFixture) -> bool {
        let positive_float = PositiveFloat::try_from(valid_number.0);
        assert_ok_eq!(positive_float, PositiveFloat(valid_number.0));
        positive_float.is_ok()
    }

    #[quickcheck_macros::quickcheck]
    fn test_positive_float_error(invalid_number: InvalidNumberFixture) -> bool {
        let invalid_float = PositiveFloat::try_from(invalid_number.0);
        invalid_float.is_err()
    }

    #[test]
    fn test_single_interest_to_interest_rates() {
        let interest_rates = Interest::Single(0.5).to_interest_rates(4);
        assert_eq!(interest_rates.len(), 4);
        assert!(!interest_rates.is_empty());

        let interest_rates = Interest::Single(0.5).to_interest_rates(0);
        assert_eq!(interest_rates.len(), 0);
        assert!(interest_rates.is_empty())
    }

    #[test]
    fn test_multiple_interest_to_interest_rates() {
        let interest_rates = Interest::Multiple(vec![0.5, 0.0, 0.2]);
        assert_eq!(interest_rates.to_interest_rates(1), vec![0.5, 0.0, 0.2])
    }

    #[test]
    fn test_distribution_to_interest_rates() {
        let interest = Interest::Distribution("sp500".to_string());
        assert_eq!(interest.to_interest_rates(3).len(), 3);
        // TODO: Test distribution doesn't exist
    }
}
