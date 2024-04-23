// Remember to import `sea_orm_migration::schema::*` schema helpers into scope
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create().table(Member::Table)
            .if_not_exists()
            .col(pk_auto(Member::Id))
            .col(uuid(Member::Pid))
            .col(string(Member::Title))
            .col(string(Member::Text))
            .col(string(Member::Email))
            .col(string(Member::Password))
            .to_owned(),
        )
        .await?;

        manager.create_table(Table::create().table(Ingredient::Table)
            .if_not_exists()
            .col(pk_auto(Ingredient::Id))
            .col(uuid(Ingredient::Pid))
            .col(string(Ingredient::Title))
            .col(string(Ingredient::Text))
            .col(integer(Ingredient::MemberId))
            .foreign_key(
                ForeignKey::create()
                .from(Ingredient::Table, Ingredient::MemberId)
                .to(Member::Table, Member::Id))
            .to_owned(),
        ).await?;

        manager.create_table(Table::create().table(Recipe::Table)
            .if_not_exists()
            .col(pk_auto(Recipe::Id))
            .col(uuid(Recipe::Pid))
            .col(string(Recipe::Title))
            .col(string(Recipe::Text))
            .to_owned(),
        ).await?;

        manager.create_table(Table::create().table(RecipeMember::Table)
            .if_not_exists()
            .col(integer(RecipeMember::MemberId))
            .col(integer(RecipeMember::RecipeId))
            .primary_key(Index::create()
                .col(RecipeMember::MemberId)
                .col(RecipeMember::RecipeId))
            .to_owned()
        ).await?;

        manager.create_table(Table::create().table(RecipeIngredient::Table)
            .if_not_exists()
            .col(integer(RecipeIngredient::RecipeId))
            .col(integer(RecipeIngredient::IngredientId))
            .col(integer(RecipeIngredient::Quantity))
            .primary_key(Index::create()
                .col(RecipeIngredient::RecipeId)
                .col(RecipeIngredient::IngredientId))
            .to_owned()
        ).await?;

        manager.create_table(
            Table::create().table(RecipeStep::Table)
            .if_not_exists()
            .col(pk_auto(RecipeStep::Id))
            .col(uuid(RecipeStep::Pid))
            .col(integer(RecipeStep::RecipeId))
            .col(integer(RecipeStep::MemberId))
            .col(string(RecipeStep::Title))
            .col(string(RecipeStep::Text))
            .foreign_key(ForeignKey::create()
                .from(RecipeStep::Table, RecipeStep::MemberId)
                .to(Member::Table, Member::Id)
            )
            .foreign_key(ForeignKey::create()
                .from(RecipeStep::Table, RecipeStep::RecipeId)
                .to(Recipe::Table, Recipe::Id)
            )
            .to_owned()
        ).await?;

        manager.create_table(Table::create().table(MediaPicture::Table)
            .if_not_exists()
            .col(pk_auto(MediaPicture::Id))
            .col(uuid(MediaPicture::Pid))
            .col(string(MediaPicture::Path))
            .col(string(MediaPicture::Thumbnail))
            .col(integer(MediaPicture::Width))
            .col(integer(MediaPicture::Height))
            .to_owned()
        ).await?;

        manager.create_table(Table::create().table(MediaVideo::Table)
            .if_not_exists()
            .col(pk_auto(MediaVideo::Id))
            .col(uuid(MediaVideo::Pid))
            .col(string(MediaVideo::Path))
            .col(string(MediaVideo::Thumbnail))
            .col(integer(MediaVideo::Width))
            .col(integer(MediaVideo::Height))
            .to_owned()
        ).await?;
    
        manager.create_table(Table::create().table(RecipeStepMediaPicture::Table)
            .col(integer(RecipeStepMediaPicture::MediaId))
            .col(integer(RecipeStepMediaPicture::RecipeStepId))
            .primary_key(Index::create()
                .col(RecipeStepMediaPicture::MediaId)
                .col(RecipeStepMediaPicture::RecipeStepId))
            .foreign_key(ForeignKey::create()
                .from(RecipeStepMediaPicture::Table, RecipeStepMediaPicture::MediaId)
                .to(MediaPicture::Table, MediaPicture::Id)
            )
            .foreign_key(ForeignKey::create()
                .from(RecipeStepMediaPicture::Table, RecipeStepMediaPicture::RecipeStepId)
                .to(RecipeStep::Table, RecipeStep::Id)
            )
            .to_owned()
        ).await?;
        
        manager.create_table(Table::create().table(RecipeStepMediaVideo::Table)
            .col(integer(RecipeStepMediaVideo::MediaId))
            .col(integer(RecipeStepMediaVideo::RecipeStepId))
            .primary_key(Index::create()
                .col(RecipeStepMediaVideo::MediaId)
                .col(RecipeStepMediaVideo::RecipeStepId))
            .foreign_key(ForeignKey::create()
                .from(RecipeStepMediaVideo::Table, RecipeStepMediaVideo::MediaId)
                .to(MediaVideo::Table, MediaVideo::Id)
            )
            .foreign_key(ForeignKey::create()
                .from(RecipeStepMediaVideo::Table, RecipeStepMediaVideo::RecipeStepId)
                .to(RecipeStep::Table, RecipeStep::Id)
            )
            .to_owned()
        ).await?;

        manager.create_table(Table::create().table(IngredientMediaVideo::Table)
            .col(integer(IngredientMediaVideo::MediaId))
            .col(integer(IngredientMediaVideo::IngredientId))
            .primary_key(Index::create()
                .col(IngredientMediaVideo::MediaId)
                .col(IngredientMediaVideo::IngredientId))
            .foreign_key(ForeignKey::create()
                .from(IngredientMediaVideo::Table, IngredientMediaVideo::MediaId)
                .to(MediaVideo::Table, MediaVideo::Id)
            )
            .foreign_key(ForeignKey::create()
                .from(IngredientMediaVideo::Table, IngredientMediaVideo::IngredientId)
                .to(Ingredient::Table, Ingredient::Id)
            )

            .to_owned()
        ).await?;


        manager.create_table(Table::create().table(IngredientMediaPicture::Table)
            .col(integer(IngredientMediaPicture::MediaId))
            .col(integer(IngredientMediaPicture::IngredientId))
            .primary_key(Index::create()
                .col(IngredientMediaPicture::MediaId)
                .col(IngredientMediaPicture::IngredientId))
            .foreign_key(ForeignKey::create()
                .from(IngredientMediaPicture::Table, IngredientMediaPicture::MediaId)
                .to(MediaPicture::Table, MediaPicture::Id)
            )
            .foreign_key(ForeignKey::create()
                .from(IngredientMediaPicture::Table, IngredientMediaPicture::IngredientId)
                .to(Ingredient::Table, Ingredient::Id)
            )
            .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(IngredientMediaVideo::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(IngredientMediaPicture::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(RecipeMember::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(RecipeIngredient::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(RecipeStepMediaPicture::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(RecipeStepMediaVideo::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(MediaVideo::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(MediaPicture::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(RecipeStep::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Recipe::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Ingredient::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Member::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Member {
    Table,
    Id,
    Pid,
    Title,
    Text,
    Email,
    Password,
}

#[derive(DeriveIden)]
enum Ingredient {
    Table,
    Id,
    Pid,
    Title,
    Text,
    MemberId,
}

#[derive(DeriveIden)]
enum Recipe {
    Table,
    Id,
    Pid,
    Title,
    Text
}

#[derive(DeriveIden)]
enum RecipeMember {
    Table,
    RecipeId,
    MemberId,
}

#[derive(DeriveIden)]
enum RecipeIngredient {
    Table,
    RecipeId,
    IngredientId,
    Quantity,
}

#[derive(DeriveIden)]
enum RecipeStep {
    Table,
    Id,
    Pid,
    Title,
    Text,
    RecipeId,
    MemberId,
}

#[derive(DeriveIden)]
enum RecipeStepMediaPicture {
    Table,
    RecipeStepId,
    MediaId
}

#[derive(DeriveIden)]
enum RecipeStepMediaVideo {
    Table,
    RecipeStepId,
    MediaId
}

#[derive(DeriveIden)]
enum IngredientMediaPicture {
    Table,
    IngredientId,
    MediaId
}

#[derive(DeriveIden)]
enum IngredientMediaVideo {
    Table,
    IngredientId,
    MediaId
}

#[derive(DeriveIden)]
enum MediaPicture {
    Table,
    Id,
    Pid,
    Path,
    Width,
    Height,
    Thumbnail,
}

#[derive(DeriveIden)]
enum MediaVideo {
    Table,
    Id,
    Pid,
    Path,
    Width,
    Height,
    Thumbnail,
}
