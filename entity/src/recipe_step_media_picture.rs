//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "recipe_step_media_picture")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub media_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub recipe_step_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::media_picture::Entity",
        from = "Column::MediaId",
        to = "super::media_picture::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    MediaPicture,
    #[sea_orm(
        belongs_to = "super::recipe_step::Entity",
        from = "Column::RecipeStepId",
        to = "super::recipe_step::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    RecipeStep,
}

impl Related<super::media_picture::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MediaPicture.def()
    }
}

impl Related<super::recipe_step::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeStep.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}