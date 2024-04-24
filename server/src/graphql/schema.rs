use async_graphql::{EmptySubscription, Schema};
use entity::async_graphql;

use crate::graphql::{mutation::Mutation, query::Query};

use entity::sea_orm::DatabaseConnection;

use std::sync::Arc;

pub type GraphQlSchema = Schema<Query, Mutation, EmptySubscription>;
pub async fn build_schema<'a>(db: Arc<DatabaseConnection>) -> GraphQlSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish()
}
