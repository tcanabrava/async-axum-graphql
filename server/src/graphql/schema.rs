use async_graphql::{EmptySubscription, Schema};
use entity::async_graphql;

use crate::{
    graphql::{
        mutation::Mutation,
        query::Query
    }, 
    state::AppState
};

use std::sync::Arc;

pub type GraphQlSchema = Schema<Query, Mutation, EmptySubscription>;
pub async fn build_schema<'a>(db: Arc<AppState>) -> GraphQlSchema {
    Schema::build(
        Query::default(),
        Mutation::default(),
        EmptySubscription)
        .data(db)
        .finish()
}