use sea_orm_migration::prelude::*;

use crate::{m20241020_021506_init_role::Role, m20241020_021519_init_permission::Permission};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RolePermission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RolePermission::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RolePermission::RoleId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RolePermission::PermissionId)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role_permission-role_id")
                            .from(RolePermission::Table, RolePermission::RoleId)
                            .to(Role::Table, Role::RoleId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role_permission-permission_id")
                            .from(RolePermission::Table, RolePermission::PermissionId)
                            .to(Permission::Table, Permission::PermissionId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RolePermission::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RolePermission {
    Table,
    Id,
    RoleId,
    PermissionId,
}
