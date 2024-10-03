pub mod router;

pub async fn serve() {
    let port = utils::env::get_env::<String>("API_PORT");
    let address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, router::app::init()).await.unwrap();
}