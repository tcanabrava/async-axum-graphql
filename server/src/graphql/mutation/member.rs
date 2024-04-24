use argon2::password_hash::rand_core::OsRng;
use async_graphql::{Context, Object, Result};

use entity::async_graphql::{self, InputObject, SimpleObject};

use entity::member;

use entity::sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use std::sync::Arc;
use uuid;

use argon2::{
    Argon2,
    PasswordHasher,
    password_hash::SaltString
};

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
        let salt = SaltString::generate(&mut OsRng);
        let argon_params = argon2::Params::new(15000, 2, 1, None)?;

        let hasher = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon_params);
        
        let password_hash:String = hasher.hash_password(input.password.as_bytes(), &salt)
            .unwrap()
            .to_string();
    
        let member = member::ActiveModel {
            email: Set(input.email),
            password: Set(password_hash),
            title: Set(input.name),
            pid: Set(uuid::Uuid::new_v4()),
            description: Set("".to_owned()),
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
