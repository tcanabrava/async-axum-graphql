use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use dotenv;

use sea_orm::{ConnectOptions, Database};
use std::sync::Arc;
use std::time::Duration;

use entity::{
    // Re-Exports, so we use the same lib
    sea_orm,
};
use serde::Serialize;

mod graphql;

mod common;
use common::AppState;

#[derive(Serialize)]
pub struct DatabaseInsertion {
    status: u16,
    text: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let database_url = dotenv::var("DATABASE_URL").unwrap();
    let mut opt = ConnectOptions::new(database_url);

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = match Database::connect(opt).await {
        Ok(conn) => Arc::new(conn),
        Err(err) => {
            panic!("{:?}", err)
        }
    };

    let state = Arc::new(AppState {
        db: db.clone(),
        schema: graphql::schema::build_schema(db).await,
    });

    let app = Router::new()
        .route("/v1/graphql", get(graphql_playground))
        .route("/v1/graphql", post(graphql_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[axum::debug_handler]
async fn graphql_handler(
    State(state): State<Arc<AppState>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let res = state.schema.execute(req.into_inner()).await;
    res.into()
}

#[axum::debug_handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/v1/graphql",
    )))
}
