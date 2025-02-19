use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let base_url = "http://localhost:3331";

    let response = client
        .post(format!("{}/rooms", base_url))
        .json(&json!({
            "name": "Living Room"
        }))
        .send()
        .await?;
    println!("Create room status: {}", response.status());

    let response = client
        .post(format!("{}/rooms/Living Room/devices", base_url))
        .json(&json!({
            "name": "Socket1",
            "description": "Living room main socket",
            "power": 100
        }))
        .send()
        .await?;
    println!("Create device status: {}", response.status());

    let response = client.get(format!("{}/rooms", base_url)).send().await?;
    let rooms: serde_json::Value = response.json().await?;
    println!("Rooms: {}", serde_json::to_string_pretty(&rooms)?);

    let response = client
        .get(format!("{}/rooms/Living Room/devices", base_url))
        .send()
        .await?;
    let devices: serde_json::Value = response.json().await?;
    println!("Devices: {}", serde_json::to_string_pretty(&devices)?);

    let response = client.get(format!("{}/report", base_url)).send().await?;
    let report = response.text().await?;
    println!("House Report:\n{}", report);

    let response = client
        .delete(format!("{}/rooms/Living Room/devices/Socket1", base_url))
        .send()
        .await?;
    println!("Delete device status: {}", response.status());

    let response = client
        .delete(format!("{}/rooms/Living Room", base_url))
        .send()
        .await?;
    println!("Delete room status: {}", response.status());

    Ok(())
}
