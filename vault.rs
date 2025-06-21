use chrono::prelude::*;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Secret {
    pub username: String,
    pub password: String,
    pub created_at: String,
}

const FILE_PATH: &str = "password.json";

pub fn setup_database() {
    let path = Path::new(FILE_PATH);
    if !path.exists() {
        let file = OpenOptions::new().write(true).create_new(true).open(path);
        match file {
            Ok(mut f) => {
                f.write(b"[]").expect("failed to initialize the json db");
            }
            Err(e) => panic!("Failed to create password.json: {}", e),
        }
    }
}

pub fn add(website: String, username: String, password: String) {
    let s = Secret {
        username,
        password,
        created_at: Local::now().to_rfc2822(),
    };

    let mut secrets = read_all_secrets();
    secrets.insert(website, s);

    let json = serde_json::to_string_pretty(&secrets).expect("failed to serialize the secret");
    std::fs::write(FILE_PATH, json).expect("failed to add password");
}

fn read_all_secrets() -> HashMap<String, Secret> {
    let path = Path::new(FILE_PATH);
    if !path.exists() {
        return HashMap::new();
    }

    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("failed to read existing database");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read database file");

    serde_json::from_str(&contents).unwrap_or_else(|_| HashMap::new())
}

pub fn get_all() -> String {
    let secrets = read_all_secrets();
    serde_json::to_string_pretty(&secrets).expect("failed to prettify secrets")
}

pub fn search(k: &str) -> Option<Secret> {
    let secrets = read_all_secrets();
    secrets.get(k).cloned()
}

pub fn delete(k: &str) {
    let mut secrets = read_all_secrets();
    secrets.remove(k);

    let json = serde_json::to_string_pretty(&secrets).expect("failed to serialize the secret");
    std::fs::write(FILE_PATH, json).expect("failed to update the database");
}
