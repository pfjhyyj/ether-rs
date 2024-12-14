pub mod modules;

#[tokio::main]
pub async fn start() -> std::io::Result<()> {
    utils::env::load_config();
    utils::logger::init();
    utils::db::init_db().await;
    utils::redis::init_redis();

    let port = utils::env::get_env::<String>("API_PORT");
    let address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, modules::get_router()).await.unwrap();
    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        tracing::error!("Failed to start server: {:?}", err);
    }
}