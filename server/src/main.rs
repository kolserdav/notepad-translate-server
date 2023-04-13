// libretranslate --host 192.168.0.3  --api-keys --req-limit 100 --require-api-key-secret --disable-web-ui --threads 4
mod prelude;
mod routes;

use routes::*;

mod locales;
use once_cell::sync::Lazy;
use prelude::*;

#[macro_use]
extern crate log;
use axum::{
    routing::{get, post},
    Router,
};

pub static LOCALES: Lazy<Vec<LocaleReviews>> = Lazy::new(|| get_translate_reviews());

#[tokio::main]
async fn main() {
    env_logger::builder().format_timestamp(None).init();

    dotenv_init().expect(".env file not found");

    let app = Router::new()
        .route("/reviews", get(stat::locale_rewiews))
        .route("/locale", get(stat::get_locale_values));

    let addr = get_address();

    debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
