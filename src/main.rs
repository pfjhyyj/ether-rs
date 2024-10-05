#[tokio::main]
async fn main() {
    utils::env::load_config();
    utils::logger::init();
    utils::db::init_db().await;
    utils::redis::init_redis();

    api::serve().await;
}
