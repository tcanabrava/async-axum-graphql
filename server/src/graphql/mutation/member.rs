use async_graphql::{Context, Object, Result};

use entity::async_graphql::{self, InputObject, SimpleObject};

use entity::member;

use entity::sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use std::sync::Arc;
use uuid;

#[derive(InputObject)]
pub struct CreateMemberInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(SimpleObject)]
pub struct DeleteResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Default)]
pub struct MemberMutation;

#[Object]
impl MemberMutation {
    pub async fn create_member(
        &self,
        ctx: &Context<'_>,
        input: CreateMemberInput,
    ) -> Result<member::Model> {
        let state = ctx.data::<Arc<DatabaseConnection>>()?;
        let member = member::ActiveModel {
            email: Set(input.email),
            password: Set(input.password),
            title: Set(input.name),
            pid: Set(uuid::Uuid::new_v4()),
            text: Set("".to_owned()),
            ..Default::default()
        };
        let res = member.insert(state.as_ref()).await?;
        Ok(res)
    }

    pub async fn delete_member(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let state = ctx.data::<Arc<DatabaseConnection>>()?;
        let res = member::Entity::delete_by_id(id)
            .exec(state.as_ref())
            .await?;

        if res.rows_affected != 1 {
            return Err(async_graphql::Error::new("Invalid number of rows affected"));
        }

        Ok(DeleteResult {
            success: true,
            rows_affected: 0,
        })
    }
}
