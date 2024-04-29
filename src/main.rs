mod arguments;
mod config;
mod n2yo;

use serde_json;


fn main() {
    let cfg = config::get_config();
    let args = arguments::get_arguments();
    let client = n2yo::N2YOClient::new(cfg.clone());

    let id = args.value_of("id").unwrap().to_string();
    let seconds = args.value_of("seconds").unwrap().to_string();

    let response = client.get_position(id, seconds);
    match response {
        Ok(ref data) => {
            let positions = data.get_positions();
            for position in positions {
                println!("{}", serde_json::to_string(&position).unwrap());
            }
        },
        Err(err) => {
            eprintln!("ERROR: {:?}", err);
            std::process::exit(1);
        },
    }
}
