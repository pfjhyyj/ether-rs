mod common;
mod router;
mod controller;
mod modules;


#[tokio::main]
async fn main() {
    utils::env::load_config();

    let app = router::get_router();

    let port = utils::env::get_env::<String>("API_PORT");
    let address = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
