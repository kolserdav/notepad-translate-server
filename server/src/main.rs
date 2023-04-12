// libretranslate --host 192.168.0.3  --api-keys --req-limit 100 --require-api-key-secret --disable-web-ui --threads 4
mod prelude;

use once_cell::sync::Lazy;
use prelude::*;

#[macro_use]
extern crate log;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub static LOCALES: Lazy<Vec<Locale>> = Lazy::new(|| get_translate_reviews());

#[tokio::main]
async fn main() {
    env_logger::builder().format_timestamp(None).init();

    dotenv_init().expect(".env file not found");

    let app = Router::new()
        .route("/", get(root))
        .route("/reviews", get(locales))
        .route("/users", post(create_user));

    let addr = get_address();

    debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn locales() -> (StatusCode, Json<Vec<&'static Locale>>) {
    let locales = &LOCALES;

    let mut result: Vec<&Locale> = vec![];
    for locale in locales.iter() {
        result.push(locale);
    }

    (StatusCode::OK, Json(result))
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
