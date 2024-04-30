mod arguments;
mod config;
mod n2yo;

use std::io::prelude::*;
use std::process::exit;


fn write_to_file(fqpn: String, data: String) -> Result<(), std::io::Error> {
    let file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(fqpn);
    match file {
        Ok(mut fp) => {
            writeln!(fp, "{}", data)
        },
        Err(err) => {
            eprintln!("ERROR: {:?}", err);
            exit(1);
        } 
    }
}


fn main() {
    let cfg = config::get_config();
    let client = n2yo::N2YOClient::new(&cfg);

    let args = arguments::get_arguments();
    let id = match args.value_of("id") {
        Some(value) => value.to_string(),
        None => {
            eprintln!("ERROR: No ID found in arguments.");
            exit(1);
        },
    };
    let seconds = match args.value_of("seconds") {
        Some(value) => value.to_string(),
        None => {
            eprintln!("ERROR: No seconds found in arguments.");
            exit(1);
        },
    };
    let filename = match args.value_of("filename") {
        Some(value) => value.to_string(),
        None => {
            eprintln!("ERROR: No filename found in arguments.");
            exit(1);
        },
    };
    let datadir = cfg.get_datadir();
    let sleep = cfg.get_sleep();

    loop {
        let response = client.get_position(&id, &seconds);
        match response {
            Ok(ref data) => {
                let positions = data.get_positions();
                for position in &positions {
                    let pos = match serde_json::to_string(&position) {
                        Ok(value) => value,
                        Err(err) => {
                            eprintln!("ERROR: {:?}", err);
                            exit(1);
                        },
                    };
                    match write_to_file(format!("{}/{}", datadir, filename), pos) {
                        Ok(_) => {},
                        Err(err) => {
                            eprintln!("ERROR: {:?}", err);
                            exit(1);
                        },
                    }
                }
            },
            Err(err) => {
                eprintln!("ERROR: {:?}", err);
                exit(1);
            },
        }
        println!("Going to sleep for {} seconds.", sleep);
        std::thread::sleep(std::time::Duration::from_secs(sleep));
    }
}
