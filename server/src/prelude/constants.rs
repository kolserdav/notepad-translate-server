use once_cell::sync::Lazy;

pub static PORT: Lazy<u16> = Lazy::new(|| {
    std::env::var("PORT")
        .expect("PORT must be set.")
        .parse::<u16>()
        .expect("PORT must be integer")
});

pub static HOST: Lazy<String> = Lazy::new(|| std::env::var("HOST").expect("HOST must be set."));

pub static LOCALES_PATH: Lazy<String> =
    Lazy::new(|| std::env::var("LOCALES_PATH").expect("LOCALES_PATH must be set."));
