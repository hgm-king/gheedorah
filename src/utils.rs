use chrono::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

#[cfg(feature = "mocks")]
use mocktopus::macros::*;

pub fn now() -> chrono::naive::NaiveDateTime {
    Utc::now().naive_local()
}

pub fn read_file_to_string(path: &String) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_file_to_bytes(path: &String) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut contents = Vec::<u8>::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

#[cfg_attr(feature = "mocks", mockable)]
pub fn gen_uuid() -> String {
    Uuid::new_v4().to_simple().to_string()
}
