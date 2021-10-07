//! An example showing off the usage of `Deserialize` to automatically decode
//! TOML into a Rust `struct`

use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
struct Config {
    global_string: Option<String>,
    num_ports: Option<u64>,
    lin_ports: Vec<LinConfig>,
}

#[derive(Debug, Deserialize)]
struct LinConfig {
    uart: String,
    rib_id: u64,
}

fn main() {
    let mut file = File::open("lin_config.toml").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    // println!("{}", contents);

    let decoded: Config = toml::from_str(&contents).unwrap();
    // println!("{:#?}", decoded);

    for port in decoded.lin_ports {
        println!("port:{:#?} rib_id:{:?}", port.uart, port.rib_id);
    }
}
