mod android;
mod server;
mod village;
mod adb;
mod config;
mod instance;
mod auto_clash_bot;


use crate::auto_clash_bot::AutoClashBot;


use std::process::{Command, Output};

fn main() {
    let mut acb = AutoClashBot::new();
    acb.run();
}

/*

let contents = fs::read_to_string("config.json").unwrap();
    let test: config::Config = serde_json::from_str(&contents).unwrap();
    println!
    ("{}", test.instance_configs[0].instance_name);

let virtual_device = android::VirtualDevice::new(String::from("Pie64"));
    let adb_client = adb::AdbClient::new();
    let serial_number = format!("localhost:{}", virtual_device.adb_port);
    adb_client.connect(serial_number.as_str());
    let adb_device = adb_client.device(serial_number).unwrap();
    adb_device.start_app("com.supercell.clashofclans");
 */