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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_positive_float_creation() {
        let positive_float = PositiveFloat::try_from(10.0);
        assert_ok_eq!(positive_float, PositiveFloat(10.0));
    }

    #[test]
    fn test_positive_float_error() {
        let invalid_float = PositiveFloat::try_from(-10.0);
        assert_err!(invalid_float);
    }
}
