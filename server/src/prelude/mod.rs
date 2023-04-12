pub mod constants;
use constants::*;
use dotenvy::{from_path, Result as DotenvResult};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs::{read_dir, DirEntry, File};
use std::io::{Error, Read};
use std::net::SocketAddr;
use std::str::from_utf8;
use std::{env::current_dir, ffi::OsStr, path::PathBuf};

pub fn dotenv_init() -> DotenvResult<()> {
    let path = PathBuf::from(file!());

    let dir = path.parent().unwrap().parent().unwrap().parent().unwrap();

    let path_raw = format!("{:?}/{:?}/.env", current_dir().unwrap(), dir).replace("\"", "");
    let dotenv_f = OsStr::new(&path_raw);

    info!("Dotenv file: {:?}", &dotenv_f);

    from_path(dotenv_f)
}

pub fn get_address() -> SocketAddr {
    let s: SocketAddr = format!("{}:{}", Lazy::force(&HOST), Lazy::force(&PORT))
        .parse()
        .expect("Failed parse host");
    s
}

pub fn get_translate_reviews() -> Vec<Locale> {
    let dir = read_dir(LOCALES_PATH.clone()).expect("Failed read locales directory");

    let mut res: Vec<Locale> = vec![];

    for file in dir.collect::<Vec<Result<DirEntry, Error>>>() {
        let item_path = file.unwrap().path();
        let meta_path = item_path.join("meta.json".to_string());

        let file = File::open(&meta_path);
        if let Err(_) = file {
            debug!("Skiped to read the meta file of locale: {:?}", meta_path);
            continue;
        }
        let mut file = file.unwrap();

        let mut buf: Vec<u8> = vec![];
        let meta_data = file.read_to_end(&mut buf);
        if let Err(e) = meta_data {
            error!(
                "Failed to read the meta file of locale: {:?}, {:?}",
                &meta_path, e
            );
            continue;
        }

        let data = from_utf8(&buf).unwrap();
        let meta_json = from_str::<LocaleMeta>(data);
        if let Err(e) = meta_json {
            error!(
                "Failed to parse the meta file of locale: {:?}, {:?}",
                &meta_path, e
            );
            continue;
        }
        let meta = meta_json.unwrap();

        res.push(Locale {
            code: item_path.file_name().unwrap().to_str().unwrap().to_string(),
            meta,
        });
    }
    res
}

#[derive(Deserialize, Debug, Serialize)]
pub struct LocaleMeta {
    pub name: String,
    pub reviewed: bool,
}

#[derive(Debug, Serialize)]
pub struct Locale {
    pub code: String,
    pub meta: LocaleMeta,
}
