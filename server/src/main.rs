use axum::{
    extract::{Path, Json},
    routing::{get,post},
    Router,
};
use dotenv;

use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use sqlx::database;

use std::time::Duration;
use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize)]
struct Ingredient {
    name: String,
    description: String
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Member {
    name: String,
    avatar: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Vote {
    up: u64,
    down: u64
}

#[derive(serde::Serialize, serde::Deserialize)]
struct RecipeStep {
    name: String,
    description: String,
    time: Duration,
    votes: Vote,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Recipe {
    name: String,
    decription: String,
    steps: Vec<RecipeStep>,
    votes: Vote,
}
struct AppState {
    db: DatabaseConnection
}

pub mod v1 {
    use super::*;

    pub async fn ok() -> &'static str {
        "Ok\n"
    }

    pub async fn get_ingredient(Path(ingredient_id): Path<String>) {
    }

    pub async fn post_ingredient(Json(payload): Json<Ingredient>) {
    }

    pub async fn get_recipe(Path(recipe_id): Path<String>) {
    }

    pub async fn post_recipe(Json(payload): Json<Recipe>) {
    }

    pub async fn get_member(Path(member_id): Path<String>) {
    }

    pub async fn add_member(Json(payload): Json<Member>) {
    }
}



#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .with_test_writer()
    .init();

    dotenv::dotenv().ok();
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
        .route("/v1/ingredients", post(v1::post_ingredient))
        .route("/v1/ingredients/:ingredient_id", get(v1::get_ingredient))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
