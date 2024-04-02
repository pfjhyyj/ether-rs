mod common;
mod router;
mod controller;


#[tokio::main]
async fn main() {
    utils::env::load_config();

    let app = router::get_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
