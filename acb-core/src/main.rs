mod android;
mod server;
mod village;
mod adb;
mod config;
mod instance;

use std::{fs, thread, sync::mpsc::{Sender, Receiver}};

use instance::Instance;
use std::sync::mpsc;
use std::io;

#[tokio::main]
async fn main() {
    let config_string = fs::read_to_string("config.json").unwrap();
    let conf: config::Config = serde_json::from_str(&config_string).unwrap();

    for instance_config in conf.instance_configs {

        println!("Instance: {} created!", instance_config.instance_name);
        let mut _instance = Instance::new(
            instance_config.server_port,
            conf.bluestacks_exe_path.clone(),
            conf.bluestacks_config_path.clone(), 
            instance_config
        ).await;

        
        tokio::spawn(async move {
            _instance.run().await;
        });   
    }

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
}

/*

let contents = fs::read_to_string("config.json").unwrap();
    let test: config::Config = serde_json::from_str(&contents).unwrap();
    println!("{}", test.instance_configs[0].instance_name);

let virtual_device = android::VirtualDevice::new(String::from("Pie64"));
    let adb_client = adb::AdbClient::new();
    let serial_number = format!("localhost:{}", virtual_device.adb_port);
    adb_client.connect(serial_number.as_str());
    let adb_device = adb_client.device(serial_number).unwrap();
    adb_device.start_app("com.supercell.clashofclans");
 */