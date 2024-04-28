use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name="user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub username: String,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    
}

impl ActiveModelBehavior for ActiveModel {}