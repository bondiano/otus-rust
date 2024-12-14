use smart_home_async::devices::socket::SmartSocketServer;

#[tokio::main]
pub async fn main() {
    let socket_server = SmartSocketServer::init("127.0.0.1:4322", "Alisa's socket", "Smart socket")
        .await
        .expect("Failed to initialize smart socket server");

    socket_server.listen().await;
}
