mod ru;
use ru::LANG as ru_lang;
use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct Locale {
    pub translate: &'static str,
}

pub fn get_locale(value: String) -> Locale {
    debug!("Locale: {}", value);
    ru_lang
}
