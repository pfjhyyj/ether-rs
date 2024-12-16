use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Menu {
    Table,
    MenuId,
    ParentId,
    Name,
    MenuType,
    Icon,
    Path,
    Sort,
    Extra,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Menu::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Menu::MenuId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Menu::ParentId)
                            .big_integer()
                            .null()
                    )
                    .col(
                        ColumnDef::new(Menu::Name)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Menu::MenuType)
                            .integer()
                            .not_null()
                            .default(0)
                    )
                    .col(
                        ColumnDef::new(Menu::Icon)
                            .string()
                            .null()
                    )
                    .col(
                        ColumnDef::new(Menu::Path)
                            .string()
                            .null()
                    )
                    .col(
                        ColumnDef::new(Menu::Sort)
                            .integer()
                            .default(0)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Menu::Extra)
                            .json()
                            .null()
                    )
                    .col(
                        ColumnDef::new(Menu::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Menu::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Menu::Table).to_owned())
            .await
    }
}