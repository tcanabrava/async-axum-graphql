use async_graphql::{Context, Object, Result};

use std::sync::Arc;

use entity::{
    async_graphql, member,
    sea_orm::{DatabaseConnection, EntityTrait},
};

#[derive(Default)]
pub struct MemberQuery;

#[Object]
impl MemberQuery {
    async fn get_members(&self, ctx: &Context<'_>) -> Result<Vec<member::Model>> {
        let state = ctx.data::<Arc<DatabaseConnection>>()?;
        let res = member::Entity::find().all(state.as_ref()).await?;
        Ok(res)
    }

    async fn get_member_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<member::Model>> {
        let state = ctx.data::<Arc<DatabaseConnection>>()?;
        let res = member::Entity::find_by_id(id).one(state.as_ref()).await?;
        Ok(res)
    }
}
