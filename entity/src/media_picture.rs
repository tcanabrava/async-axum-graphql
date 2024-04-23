//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "media_picture")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub path: String,
    pub thumbnail: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ingredient_media_picture::Entity")]
    IngredientMediaPicture,
    #[sea_orm(has_many = "super::recipe_step_media_picture::Entity")]
    RecipeStepMediaPicture,
}

impl Related<super::ingredient_media_picture::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IngredientMediaPicture.def()
    }
}

impl Related<super::recipe_step_media_picture::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecipeStepMediaPicture.def()
    }
}

impl Related<super::ingredient::Entity> for Entity {
    fn to() -> RelationDef {
        super::ingredient_media_picture::Relation::Ingredient.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::ingredient_media_picture::Relation::MediaPicture
                .def()
                .rev(),
        )
    }
}

impl Related<super::recipe_step::Entity> for Entity {
    fn to() -> RelationDef {
        super::recipe_step_media_picture::Relation::RecipeStep.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::recipe_step_media_picture::Relation::MediaPicture
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
