pub use sea_orm_migration::prelude::*;

mod m20241002_020949_init_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20241002_020949_init_user::Migration)]
    }
}
