//@author Stanislav Polaniev <spolanyev@gmail.com>
use std::net::SocketAddr;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;

async fn get_weather(
    Path(city): Path<String>,
) -> Result<Json<ExportedWeather>, (StatusCode, Json<ErrorMessage>)> {
    let api_key = "";
    let api_url = Url::parse_with_params(
        "http://caddy/data/2.5/weather",
        &[
            ("q", city.as_str()),
            ("appid", api_key),
            ("units", "metric"),
        ],
    )
    .expect("Failed to parse url");
    let api_page = reqwest::get(api_url)
        .await
        .expect("Failed to get page")
        .text()
        .await
        .expect("Failed to get text");

    match serde_json::from_str(api_page.as_str()) {
        Ok::<ImportedWeather, _>(imported_weather) => {
            let exported_weather = ExportedWeather {
                city,
                temperature: imported_weather.main.temp,
                humidity: imported_weather.main.humidity,
                wind: imported_weather.wind.speed,
            };

            Ok(Json(exported_weather))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorMessage {
                message: "Wrong Weather API response received".to_owned(),
            }),
        )),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/weather/:city", get(get_weather));
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to serve");
}

#[derive(Debug, Deserialize)]
struct ImportedWeather {
    main: Main,
    wind: Wind,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f32,
    humidity: u8,
}

#[derive(Debug, Deserialize)]
struct Wind {
    speed: f32,
}

#[derive(Debug, Serialize)]
struct ExportedWeather {
    city: String,
    temperature: f32,
    humidity: u8,
    wind: f32,
}

#[derive(Debug, Serialize)]
struct ErrorMessage {
    message: String,
}
