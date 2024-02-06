use axum::extract;
use axum::http::StatusCode;
use axum::response;
use axum::response::IntoResponse;
use axum::routing;
use axum::Router;

use crate::distributions;
use crate::error;
use crate::investment;
use crate::investment_config;
use crate::types;

pub struct Server<'a> {
    host: String,
    port: String,
    pg_pool: &'a sqlx::PgPool,
}

impl<'a> Server<'a> {
    pub fn new(host: String, port: String, pg_pool: &'a sqlx::PgPool) -> Self {
        Self {
            host,
            port,
            pg_pool,
        }
    }

    pub async fn serve(&self) -> Result<(), error::ApplicationError> {
        self.setup_db().await?;
        let app = Router::new()
            .route("/check", routing::get(health_check))
            .route("/simulate", routing::post(get_investment_result));
        let listener = tokio::net::TcpListener::bind(format!("{}:{}", self.host, self.port))
            .await
            .unwrap();
        println!("Listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();
        Ok(())
    }

    async fn setup_db(&self) -> Result<(), error::ApplicationError> {
        println!("Setting up the DB");
        let count = sqlx::query!("SELECT count(name) FROM real_distributions")
            .fetch_one(self.pg_pool)
            .await?
            .count
            .map_or(0, |x| x);

        println!("The count is {}", count);
        if count == 0 {
            for (name, data) in distributions::get_distributions().iter() {
                sqlx::query!(
                    "INSERT INTO real_distributions (name, data) VALUES ($1, $2)",
                    name,
                    data
                )
                .execute(self.pg_pool)
                .await?;
            }
        }
        Ok(())
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

async fn health_check() -> impl response::IntoResponse {
    StatusCode::OK.into_response()
}
