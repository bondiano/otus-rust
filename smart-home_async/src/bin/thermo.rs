use smart_home_async::devices::thermo::TemperatureGeneratorServer;

#[tokio::main]
pub async fn main() {
    let thermo_generator = TemperatureGeneratorServer::init("127.0.0.1:4321")
        .await
        .expect("Failed to initialize temperature generator");

    thermo_generator.listen("127.0.0.1:4320").await;
}
