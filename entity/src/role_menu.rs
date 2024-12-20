//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "role_menu")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub role_id: i64,
    pub menu_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
