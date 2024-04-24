use crate::graphql::schema::GraphQlSchema;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub schema: GraphQlSchema,
}
