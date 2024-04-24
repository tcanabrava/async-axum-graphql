//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use async_graphql::*;

use sea_orm::entity::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "member")]
#[graphql(concrete(name = "Member", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub pid: Uuid,
    pub title: String,
    pub text: String,
    pub email: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ingredient::Entity")]
    Ingredient,
    #[sea_orm(has_many = "super::recipe_step::Entity")]
    RecipeStep,
}

impl Related<super::ingredient::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ingredient.def()
    }
}

impl Related<super::recipe_step::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeStep.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
