use chrono::FixedOffset;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name="user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub username: String,
    pub password: String,
    pub avatar: String,
    pub nickname: String,
    pub mobile: String,
    pub created_at: chrono::DateTime<FixedOffset>,
    pub updated_at: chrono::DateTime<FixedOffset>,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    
}

impl ActiveModelBehavior for ActiveModel {}