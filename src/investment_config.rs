use crate::types;

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub deposit: usize,
    pub return_rates: types::Interest,
    pub years: usize,
    pub annual_contributions: types::AnnualContribution,
}
