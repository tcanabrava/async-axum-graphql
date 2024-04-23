//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "recipe_ingredient")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub recipe_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub ingredient_id: i32,
    pub quantity: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
