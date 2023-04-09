//@author Stanislav Polaniev <spolanyev@gmail.com>

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[get("/weather/{city}")]
async fn get_weather(city: web::Path<String>) -> impl Responder {
    let city: String = city.into_inner();
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

            HttpResponse::Ok().json(exported_weather)
        }
        Err(_) => HttpResponse::InternalServerError().json(ErrorMessage {
            message: "Wrong Weather API response received".to_owned(),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_weather))
        .bind(("0.0.0.0", 80))?
        .run()
        .await
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
