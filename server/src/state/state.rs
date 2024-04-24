use crate::graphql::schema::GraphQlSchema;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub schema: GraphQlSchema,
}
