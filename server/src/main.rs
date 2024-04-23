use axum::{
    extract::{Path, Json, State},
    routing::{get,post},
    Router,
    response::{Html, IntoResponse},
    http::{StatusCode, Uri, header::{self, HeaderMap, HeaderName}},
};
use dotenv;
use std::collections::HashMap;
use sea_orm::{Database, DatabaseConnection, ConnectOptions};

use std::time::Duration;
use std::sync::Arc;
use serde::Serialize;
use entity::{prelude::*, *};

pub struct AppState {
    db: DatabaseConnection
}

#[derive(Serialize)]
pub struct DatabaseInsertion {
    status: u16,
    text: String
}

pub mod v1 {
    use sea_orm::{ActiveModelTrait, ActiveValue};

    use super::*;

    pub async fn ok() -> &'static str {
        "Ok\n"
    }

    pub async fn add_member(
        State(state): State<Arc<AppState>>,
        Json(payload): Json<entity::member::Model>)
        -> (StatusCode, Json<DatabaseInsertion>)
    {
        let mut model = member::ActiveModel::from(payload);
        model.id = ActiveValue::not_set();

        let res = model.insert(&state.db).await;

        match res {
            Ok(_) => (
                StatusCode::CREATED,
                Json(DatabaseInsertion{status: StatusCode::CREATED.as_u16(), text: "".to_owned()})),
            Err(err) => (StatusCode
                ::CONFLICT, 
                Json(DatabaseInsertion{status: StatusCode::CONFLICT.as_u16(), text: err.to_string()}))
        }
    }
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

    let db = Database::connect(opt).await.unwrap();
    let state = Arc::new(AppState{db});

    let app = Router::new()
        .route("/v1/ok", get(v1::ok))
        .route("/v1/member", post(v1::add_member))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
