use csv::ReaderBuilder;
use std::collections::HashMap;

pub fn get_distributions() -> HashMap<&'static str, Vec<f64>> {
    let mut distributions = HashMap::new();
    println!(
        "Working dir: {:?}",
        std::env::current_dir().unwrap()
    );

    let mut sp500_reader = ReaderBuilder::new()
        .from_path("real_distributions/sp500_dist.csv")
        .expect("Error loading SP&500 distribution");
    let mut msci_world_reader = ReaderBuilder::new()
        .from_path("real_distributions/msci_world_dist.csv")
        .expect("Error loading MSCI World distribution");

    let mut sp500_dist: Vec<f64> = Vec::new();
    let mut msci_world_dist: Vec<f64> = Vec::new();

    for record in sp500_reader.records().flatten() {
        let rate: f64 = record[1]
            .parse()
            .expect("Failure parsing a return interest into a f64");
        sp500_dist.push(rate / 100.0);
    }

    if let Some(old_value) = distributions.insert("sp500", sp500_dist) {
        panic!("Found an existing value for sp500: {:?}", old_value);
    }

    for record in msci_world_reader.records().flatten() {
        let rate: f64 = record[1]
            .parse()
            .expect("Failure parsing a return interest into a f64");
        msci_world_dist.push(rate / 100.0);
    }

    if let Some(old_value) = distributions.insert("msci_world", msci_world_dist) {
        panic!("Found an existing value for msci_world: {:?}", old_value);
    };

    distributions
}

#[cfg(test)]
mod test {
    use super::get_distributions;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_distributions() {
        let distributions = get_distributions();

        assert_eq!(distributions.get("sp500").unwrap().len(), 30);
        assert_eq!(distributions.get("msci_world").unwrap().len(), 44);
    }
}
