use crate::config::Config;

use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};
use std::time::Duration;


#[derive(Deserialize, Serialize)]
pub struct Info {
    pub satname: String,
    pub satid: u64,
    pub transactionscount: u64,
}

#[derive(Deserialize, Serialize)]
pub struct Position {
    pub satlatitude: f64,
    pub satlongitude: f64,
    pub sataltitude: f64,
    pub azimuth: f64,
    pub elevation: f64,
    pub ra: f64,
    pub dec: f64,
    pub timestamp: u64,
    pub eclipsed: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Satellite {
    pub info: Info,
    pub positions: Option<Vec<Position>>,
}

pub struct N2YOClient {
    client: reqwest::blocking::Client,
    config: Config,
}

impl N2YOClient {
    fn get(&self, url: String) -> Response {
        let result = self
            .client
            .get(url)
            .send();
        match result {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => response,
                _ => {
                    eprintln!("ERROR: {}", response.status());
                    std::process::exit(1);
                },
            },
            Err(err) => {
                eprintln!("ERROR: {:?}", err);
                std::process::exit(1);
            }
        }
    }
    pub fn new(config: Config) -> Self {
        let result = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(config.get_timeout()))
            .build();
        match result {
            Ok(client) => Self {
                client,
                config,
            },
            Err(err) => {
                eprintln!("ERROR: When creating blocking client: {:?}", err);
                std::process::exit(1);
            }
        }
    }
    pub fn get_position(&self, id: String, seconds: String) -> Result<Satellite, reqwest::Error> {
        let url = self.config.get_url_position(id, seconds);
        self.get(url).json::<Satellite>()
    }
}
