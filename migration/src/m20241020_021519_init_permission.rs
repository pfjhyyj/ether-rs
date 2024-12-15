use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permission::PermissionId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Permission::Object)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Permission::Action)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Permission::Name)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Permission::Description)
                            .string()
                            .null(),
                    )
                    .to_owned()
                )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permission::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Permission {
    Table,
    PermissionId,
    Object,
    Action,
    Name,
    Description,
}
