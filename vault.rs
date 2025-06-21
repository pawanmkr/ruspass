use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize)]
struct Secret {
    website: String,
    username: String,
    password: String,
    created_at: String,
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
        website,
        username,
        password,
        created_at: Local::now().to_rfc2822(),
    };

    let mut secrets: Vec<Secret> = read_all_secrets();
    secrets.push(s);

    let json = serde_json::to_string_pretty(&secrets).expect("failed to serialize the secret");
    std::fs::write(FILE_PATH, json).expect("failed to add password");
}

fn read_all_secrets() -> Vec<Secret> {
    let path = Path::new(FILE_PATH);
    if !path.exists() {
        return vec![];
    }

    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("failed to read existing database");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read database file");

    serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
}
