use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimulationError {
    #[error("Type error")]
    TypeError(#[from] TypeError),
    #[error("Computed results are invalid")]
    InvalidInvestmentResults,
}

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("PositiveFloat cannot store `{0}`")]
    PositiveFloatError(f64),
    #[error("NaN is invalid")]
    NaNInvalid,
}
