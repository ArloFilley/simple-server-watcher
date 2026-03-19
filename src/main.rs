use std::{fs, io::Write};

use sysinfo::System;

use reqwest::{self, StatusCode};
use tokio::time::{sleep, Duration};

use crate::system::SystemInfo;

pub mod disk;
pub mod process;
pub mod system;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut system = System::new_all();

    fs::create_dir("./data")?;
    let mut file = fs::File::create("./data/data.json")?;

    // ! Change key before compiling
    let key = String::from("Change this key before compiling!!!");
    // ! Change endpoint before compiling
    let endpoint = String::from("http://example.com");
    let refresh_time = Duration::from_secs_f32(5.0);

    loop {
        system.refresh_all();

        let info = SystemInfo::get(&mut system, key.clone());
        let info_json = serde_json::to_string(&info)?;

        file.write_all(info_json.as_bytes())?;
        post(info_json, &endpoint).await;
        sleep(refresh_time).await;
    }
}

async fn post(info: String, endpoint: &String) {
    let request = reqwest::Client::new()
        .post(endpoint)
        .body(info)
        .send()
        .await;

    match request {
        Err(why) => println!("Error {why}"),
        Ok(response) => match response.status() {
            StatusCode::OK => println!("Sent Data"),
            _ => println!("Unsucessful"),
        },
    }
}
