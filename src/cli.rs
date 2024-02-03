use crate::investment;
use crate::investment_config;
use crate::types;

pub fn run_cli_simulation(config_file: String) {
    let config: investment_config::Configuration = config::Config::builder()
        .add_source(config::File::new(&config_file, config::FileFormat::Json))
        .build()
        .expect("Error loading configuration file")
        .try_deserialize()
        .expect("Error deserializing the configuration");

    let investment = investment::Investment::new(
        types::PositiveFloat::try_from(config.deposit as f64).unwrap(),
        config.years,
        config
            .annual_contributions
            .to_annual_contributions(config.years),
        config.return_rates.to_interest_rates(config.years),
    );

    let investment_snapshots = investment.simulate().unwrap();
    let investment_results: Vec<investment::InvestmentSnapshotResult> = investment_snapshots
        .iter()
        .map(|snapshot| snapshot.result())
        .collect();
    for (year, result) in investment_results.iter().enumerate() {
        println!(
            "Investment result year {}\n {}",
            year + 1,
            serde_json::to_string(result).unwrap()
        );
    }
    let investment_result = investment::get_investment_result(investment_results).unwrap();
    println!(
        "Investment result\n {}",
        serde_json::to_string(&investment_result).unwrap()
    );
}
