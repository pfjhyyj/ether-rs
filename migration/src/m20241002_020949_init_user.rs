use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum User {
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
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Nickname).string().null())
                    .col(ColumnDef::new(User::Avatar).string().null())
                    .col(ColumnDef::new(User::Mobile).string().null().unique_key())
                    .col(ColumnDef::new(User::Email).string().null().unique_key())
                    .col(
                        ColumnDef::new(User::Status)
                            .small_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(User::Table)
            .columns([
                User::UserId,
                User::Username,
                User::Password,
                User::Nickname,
                User::Status,
            ])
            .values_panic([
                0.into(),
                "ether_admin".into(),
                "$2b$12$T43BHiuqzvV8DpN5TClvIeBse2dk0PBO4G9WVnFU/lY7mwcHDmnFy".into(),
                "ether admin".into(),
                0.into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}
