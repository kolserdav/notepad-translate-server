use axum::{extract::Query, http::StatusCode, Json};
use serde::Deserialize;

use crate::{
    locales::{get_locale, Locale},
    prelude::LocaleReviews,
    LOCALES,
};

pub async fn locale_rewiews() -> (StatusCode, Json<Vec<&'static LocaleReviews>>) {
    let locales = &LOCALES;

    let mut result: Vec<&LocaleReviews> = vec![];
    for locale in locales.iter() {
        result.push(locale);
    }

    (StatusCode::OK, Json(result))
}

#[derive(Deserialize)]
pub struct GetLocale {
    pub locale: String,
}

pub async fn get_locale_values(Query(payload): Query<GetLocale>) -> (StatusCode, Json<Locale>) {
    let locale = get_locale(payload.locale);
    // std::thread::sleep(std::time::Duration::from_secs(55555555555));
    (StatusCode::OK, Json(locale))
}
