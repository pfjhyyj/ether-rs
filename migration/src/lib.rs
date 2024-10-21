pub use sea_orm_migration::prelude::*;

mod m20241002_020949_init_user;
mod m20241019_115953_init_menu;
mod m20241020_021506_init_role;
mod m20241020_021519_init_permission;
mod m20241020_024905_init_role_permission;
mod m20241020_024913_init_role_menu;
mod m20241020_025112_init_user_role;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241002_020949_init_user::Migration),
            Box::new(m20241019_115953_init_menu::Migration),
            Box::new(m20241020_021506_init_role::Migration),
            Box::new(m20241020_021519_init_permission::Migration),
            Box::new(m20241020_024905_init_role_permission::Migration),
            Box::new(m20241020_024913_init_role_menu::Migration),
            Box::new(m20241020_025112_init_user_role::Migration),
        ]
    }
}
