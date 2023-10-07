#[derive(Copy, Clone)]
pub struct PositiveFloat(pub f64);

#[derive(Debug)]
pub struct PositiveFloatError(String);

impl TryFrom<f64> for PositiveFloat {
    type Error = PositiveFloatError;

    fn try_from(value: f64) -> Result<Self, PositiveFloatError> {
        if value < 0.0 {
            return Err(PositiveFloatError(format!(
                "{} is a negative float.",
                value
            )));
        }

        Ok(Self(value))
    }
}
