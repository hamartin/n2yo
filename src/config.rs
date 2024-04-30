#[derive(Clone, serde::Deserialize)]
pub struct Config {
    base_url: String,
    datadir: String,
    host: String,
    key: String,
    observer_alt: String,
    observer_lat: String,
    observer_lon: String,
    schema: String,
    sleep: u64,
    timeout: u64,
}

impl Config {
    fn get_url(&self, norad_id: &String, seconds: &String, function: String) -> String {
        format!("{}://{}{}/{}/{}/{}/{}/{}/{}/&apiKey={}", 
            self.schema,
            self.host,
            self.base_url,
            function,
            norad_id,
            self.observer_lat,
            self.observer_lon,
            self.observer_alt,
            seconds,
            self.key,
        )
    }
    pub fn get_datadir(&self) -> String {
        self.datadir.clone()
    }
    pub fn get_sleep(&self) -> u64 {
        self.sleep
    }
    pub fn get_timeout(&self) -> u64 {
        self.timeout
    }
    pub fn get_url_position(&self, norad_id: &String, seconds: &String) -> String {
        self.get_url(norad_id, seconds, String::from("positions"))
    }
}


pub fn get_config() -> Config {
    if let Some(proj_dirs) = directories::ProjectDirs::from("com", "moshwire", "n2yo") {
        let config_dir = proj_dirs.config_dir();
        let config_file = std::fs::read_to_string(config_dir.join("config.toml"));
        let config: Config = match config_file {
            Ok(ref text) => {
                let cfg = toml::from_str(text);
                match cfg {
                    Ok(ret) => ret,
                    Err(err) => {
                        eprintln!("There was an error loading the configuration file: {}", err);
                        std::process::exit(1);
                    }
                }
            }
            Err(_) => Config {
                base_url: "NoBaseURLSet".to_string(),
                datadir: "NoDataDirSet".to_string(),
                host: "NoHostSet".to_string(),
                key: "NoKeySet".to_string(),
                observer_alt: "No observer altitude set".to_string(),
                observer_lat: "No observer latitude set".to_string(),
                observer_lon: "No observer longitude set".to_string(),
                schema: "NoSchemaSet".to_string(),
                sleep: 0,
                timeout: 0,
            },
        };
        config
    } else {
        Config {
            base_url: "NoBaseURLSet".to_string(),
            datadir: "NoDataDirSet".to_string(),
            host: "NoHostSet".to_string(),
            key: "NoKeySet".to_string(),
            observer_alt: "No observer altitude set".to_string(),
            observer_lat: "No observer latitude set".to_string(),
            observer_lon: "No observer longitude set".to_string(),
            schema: "NoSchemaSet".to_string(),
            sleep: 0,
            timeout: 0,
        }
    }
}
