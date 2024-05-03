use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum User {
    Table,
    UserId,
    Username,
    Password,
    Nickname,
    Avatar,
    Mobile,
    Email,
    Status,
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
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::UserId)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null().unique_key())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Nickname).string().null())
                    .col(ColumnDef::new(User::Avatar).string().null())
                    .col(ColumnDef::new(User::Mobile).string().null().unique_key())
                    .col(ColumnDef::new(User::Email).string().null().unique_key())
                    .col(ColumnDef::new(User::Status).small_integer().not_null().default(0))
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp())
                    )
                    .to_owned()
            )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(User::Table)
                    .to_owned()
            )
        .await
    }
}
