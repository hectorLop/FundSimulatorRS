#[derive(Copy, Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod faker {
    use super::PositiveFloat;
    use fake::{Dummy, Faker};

    impl Dummy<Faker> for PositiveFloat {
        fn dummy_with_rng<R: fake::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
            Self(rng.gen_range(0.0..100000000000000000.0))
        }
    }
}

#[cfg(test)]
mod test {
    use super::PositiveFloat;
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
}
