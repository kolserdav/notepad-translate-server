mod ru;
use ru::LANG as ru_lang;
use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct Locale {
    pub settings: &'static str,
    pub translate: &'static str,
    pub select_native: &'static str,
    pub select_learn: &'static str,
}

pub fn get_locale(value: String) -> Locale {
    debug!("Locale: {}", value);
    ru_lang
}
