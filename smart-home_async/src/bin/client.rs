use smart_home_async::devices::{
    socket::{Command, SmartSocketClient},
    thermo::SmartThermoClient,
};
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
pub async fn main() {
    let mut socket_client = SmartSocketClient::init("127.0.0.1:4322")
        .await
        .expect("Expect socket client");
    let thermo_client = SmartThermoClient::init("127.0.0.1:4320")
        .await
        .expect("Expect thermo client");

    show_menu();
    loop {
        process_input(&mut socket_client, &thermo_client).await;
    }
}

fn show_menu() {
    println!();
    println!("------------------");
    println!("Select action:");
    println!("1) switch socket");
    println!("2) check socket status");
    println!("3) check temperature");
    println!("_) exit");
}

async fn process_input(socket: &mut SmartSocketClient, thermo: &SmartThermoClient) {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        match line.trim() {
            "1" => {
                socket
                    .run_command(Command::Switch)
                    .await
                    .expect("Failed to switch socket");
            }
            "2" => {
                let status = socket
                    .run_command(Command::Status)
                    .await
                    .expect("Failed to get status");

                println!("Socket status: {status}");
            }
            "3" => {
                let temperature = thermo.get_temperature().await;
                println!("Current temperature: {temperature}");
            }
            _ => {
                println!("Exiting...");
                std::process::exit(0);
            }
        }
    }
}
