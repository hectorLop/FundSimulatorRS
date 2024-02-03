use axum::extract;
use axum::response;
use axum::routing::post;
use axum::Router;

use crate::investment;
use crate::investment_config;
use crate::types;

pub struct Server {
    host: String,
    port: String,
}

impl Server {
    pub fn new(host: String, port: String) -> Self {
        Self { host, port }
    }

    pub async fn serve(&self) {
        let app = Router::new().route("/simulate", post(get_investment_result));
        let listener = tokio::net::TcpListener::bind(format!("{}:{}", self.host, self.port))
            .await
            .unwrap();
        println!("Listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();
        println!("Exiting");
    }
}

async fn get_investment_result(
    extract::Json(config): extract::Json<investment_config::Configuration>,
) -> response::Json<investment::InvestmentResult> {
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

    let investment_result = investment::get_investment_result(investment_results).unwrap();

    response::Json(investment_result)
}
