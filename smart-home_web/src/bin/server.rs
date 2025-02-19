use axum::serve::serve;
use smart_home_web::create_router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = create_router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3331));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server running on http://{}", addr);
    serve(listener, app.into_make_service()).await.unwrap();
}
