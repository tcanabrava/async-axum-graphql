use async_graphql::{
    Context,
    Object,
    Result
};

use entity::async_graphql::{
    self,
    InputObject,
    SimpleObject
};

use entity::member;

use entity::sea_orm::{
    ActiveModelTrait,
    Set
};
use sea_orm::EntityTrait;

use crate::AppState;

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
    ) -> Result<member::Model> 
    {
        let state = ctx.data::<AppState>()?;
        let member = member::ActiveModel {
            email: Set(input.email),
            password: Set(input.password),
            title: Set(input.name),
            ..Default::default()
        };
        let res = member.insert(&state.db).await?;
        Ok(res)
    }

    pub async fn delete_member(&self, ctx: &Context<'_>, id: i32)
     -> Result<DeleteResult> 
     {
        let state = ctx.data::<AppState>()?;
        let res = member::Entity::delete_by_id(id)
            .exec(&state.db)
            .await?;

        if res.rows_affected != 1 {
            return Err(async_graphql::Error::new("Invalid number of rows affected"))            
        }

        Ok(DeleteResult{success: true, rows_affected: 0})
     }
}