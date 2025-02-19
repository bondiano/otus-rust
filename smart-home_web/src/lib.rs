use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use smart_home::{
    devices::{device::Device, socket::SmartSocket},
    house::House,
    report_provider::DeviceInfoProvider,
};
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    house: Arc<RwLock<House>>,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Smart house error: {0}")]
    SmartHouse(#[from] smart_home::errors::SmartHouseError),
    #[error("Not found")]
    NotFound,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::SmartHouse(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct RoomResponse {
    name: String,
    devices: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoomRequest {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateDeviceRequest {
    name: String,
    description: String,
    power: u32,
}

pub struct MockDeviceInfoProvider;

impl DeviceInfoProvider for MockDeviceInfoProvider {
    fn get_devices(&self) -> Vec<&str> {
        vec!["Socket1", "Socket2"]
    }
}

pub fn create_router() -> Router {
    let state = AppState {
        house: Arc::new(RwLock::new(House::new("Smart House"))),
    };

    Router::new()
        .route("/rooms", get(get_rooms).post(create_room))
        .route("/rooms/{room_name}", delete(delete_room))
        .route(
            "/rooms/{room_name}/devices",
            get(get_devices).post(create_device),
        )
        .route(
            "/rooms/{room_name}/devices/{device_name}",
            delete(delete_device),
        )
        .route("/report", get(get_report))
        .with_state(state)
}

#[axum::debug_handler]
async fn get_rooms(State(state): State<AppState>) -> Json<Vec<RoomResponse>> {
    let house = state.house.read().await;
    let rooms = house
        .get_rooms()
        .map(|room| RoomResponse {
            name: room.get_name().to_string(),
            devices: room
                .get_devices()
                .map(|device| device.get_name().to_string())
                .collect(),
        })
        .collect();

    Json(rooms)
}

#[axum::debug_handler]
async fn create_room(
    State(state): State<AppState>,
    Json(request): Json<CreateRoomRequest>,
) -> Result<StatusCode, ApiError> {
    let mut house = state.house.write().await;
    house.add_room(&request.name)?;
    Ok(StatusCode::CREATED)
}

#[axum::debug_handler]
async fn delete_room(
    State(state): State<AppState>,
    Path(room_name): Path<String>,
) -> Result<StatusCode, ApiError> {
    let mut house = state.house.write().await;
    house.remove_room(&room_name)?;
    Ok(StatusCode::NO_CONTENT)
}

#[axum::debug_handler]
async fn get_devices(
    State(state): State<AppState>,
    Path(room_name): Path<String>,
) -> Result<Json<Vec<String>>, ApiError> {
    let house = state.house.read().await;
    let room = house
        .get_rooms()
        .find(|room| room.get_name() == room_name)
        .ok_or(ApiError::NotFound)?;

    Ok(Json(
        room.get_devices()
            .map(|device| device.get_name().to_string())
            .collect(),
    ))
}

#[axum::debug_handler]
async fn create_device(
    State(state): State<AppState>,
    Path(room_name): Path<String>,
    Json(request): Json<CreateDeviceRequest>,
) -> Result<StatusCode, ApiError> {
    let mut house = state.house.write().await;
    let room = house.get_room_mut(&room_name).ok_or(ApiError::NotFound)?;

    let socket = SmartSocket::new(&request.name, &request.description, request.power);
    room.add_device(socket.into())?;
    Ok(StatusCode::CREATED)
}

#[axum::debug_handler]
async fn delete_device(
    State(state): State<AppState>,
    Path((room_name, device_name)): Path<(String, String)>,
) -> Result<StatusCode, ApiError> {
    let mut house = state.house.write().await;
    let room = house.get_room_mut(&room_name).ok_or(ApiError::NotFound)?;

    room.remove_device(&device_name)?;
    Ok(StatusCode::NO_CONTENT)
}

#[axum::debug_handler]
async fn get_report(State(state): State<AppState>) -> String {
    let house = state.house.read().await;
    house.create_report(&MockDeviceInfoProvider)
}
