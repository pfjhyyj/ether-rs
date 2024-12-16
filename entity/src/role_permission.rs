//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "role_permission")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub role_id: i64,
    pub permission_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::permission::Entity",
        from = "Column::PermissionId",
        to = "super::permission::Column::PermissionId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Permission,
    #[sea_orm(
        belongs_to = "super::role::Entity",
        from = "Column::RoleId",
        to = "super::role::Column::RoleId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Role,
}

impl Related<super::permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permission.def()
    }
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
