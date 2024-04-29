//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "author")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    #[sea_orm(column_type = "Float")]
    pub rating: f32,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub cover: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::book_author::Entity")]
    BookAuthor,
}

impl Related<super::book_author::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookAuthor.def()
    }
}

impl Related<super::book::Entity> for Entity {
    fn to() -> RelationDef {
        super::book_author::Relation::Book.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::book_author::Relation::Author.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}