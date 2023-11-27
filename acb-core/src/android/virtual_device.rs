use std::fs;
use std::process::Command;
use std::thread;
use std::time::Duration;

const BLUESTACKS_EXE_PATH: &str = "C:/Program Files/BlueStacks_nxt/HD-PLAYER.exe";
const BLUESTACKS_CONFIG_PATH: &str = "C:/ProgramData/BlueStacks_nxt/bluestacks.conf";

pub struct VirtualDevice {
    pub config_path: String,
    pub instance_name: String,
    pub adb_port: u32
}

impl VirtualDevice {
    pub fn get_adb_port(config_path: &String, instance_name: &String) -> Option<u32> {
        let contents = match fs::read_to_string(config_path) {
            Ok(contents) => contents,
            Err(_) => return None
        };

        let lines = contents.split("\n");
        for line in lines {
            let key = format!("bst.instance.{}.status.adb_port", instance_name);
            if line.starts_with(&key) {
                let stripped = line.to_string().replace(&key, "");
                let number: u32 = stripped[2..stripped.len()-1].parse().unwrap();
                return Some(number);
            }
        }
        None
    }

    pub fn new(instance_name: String) -> VirtualDevice {
        VirtualDevice::start_device(&instance_name);
        thread::sleep(Duration::from_secs(10));
        let adb_port = VirtualDevice::get_adb_port(&BLUESTACKS_CONFIG_PATH.to_string(), &instance_name).unwrap();
        VirtualDevice{
            config_path: BLUESTACKS_CONFIG_PATH.to_string(),
            instance_name,
            adb_port
        }
    }

    pub fn start_device(instance_name: &String) {
        Command::new(BLUESTACKS_EXE_PATH)
            .arg("--instance")
            .arg(instance_name)
            .spawn().expect("TODO: panic message");
    }
}