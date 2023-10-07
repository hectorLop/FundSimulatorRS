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
mod test {
    use super::PositiveFloat;
    use claim::{assert_err, assert_ok_eq};
    use proptest::num::f64::{NEGATIVE, POSITIVE};
    use proptest::test_runner::TestRunner;

    #[test]
    fn test_positive_float_creation() {
        let mut runner = TestRunner::default();

        runner
            .run(&POSITIVE, |val| {
                let positive_float = PositiveFloat::try_from(val);
                assert_ok_eq!(positive_float, PositiveFloat(val));
                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn test_positive_float_error() {
        let mut runner = TestRunner::default();

        runner
            .run(&NEGATIVE, |val| {
                let invalid_float = PositiveFloat::try_from(val);
                assert_err!(invalid_float);
                Ok(())
            })
            .unwrap();
    }
}
