use std::process::Output;
use std::str;
use crate::adb::{AdbDevice, command, RustADBError};


#[derive(Clone)]
pub struct AdbClient {
    adb_path: String,
}

impl AdbClient {
    pub fn new() -> AdbClient {
        AdbClient {
            adb_path: "adb".to_string()
        }
    }

    pub fn start_cmd(&self, command: Vec<&str>) -> Result<Output, RustADBError> {
        command::start_cmd(&self.adb_path, command, None)
    }

    pub fn cmd(&self, command: Vec<&str>) -> Result<String, RustADBError> {
        let output = match self.start_cmd(command) {
            Ok(output) => output,
            Err(err) => return Err(err)
        };
        match String::from_utf8(output.stdout) {
            Ok(out) => Ok(out),
            Err(_) => Err(RustADBError::Utf8StringError)
        }
    }

    pub fn start_server(&self) -> Result<String, RustADBError> {
        self.cmd(vec!["start-server"])
    }

    pub fn kill_server(&self) -> Option<RustADBError> {
        match self.cmd(vec!["kill-server"]) {
            Ok(_) => None,
            Err(err) => Some(err)
        }
    }

    pub fn version(&self) -> Result<String, RustADBError> {
        match self.cmd(vec!["version"]) {
            Ok(output) => Ok(output.trim().split("\n").next().unwrap().to_string()),
            Err(err) => Err(err)
        }
    }

    pub fn list_devices(&self) -> Result<Vec<String>, RustADBError> {
        let mut devices: Vec<String> = vec![];
        let mut output = match self.cmd(vec!["devices"]) {
            Ok(output) => output,
            Err(err) => return Err(err)
        };
        let mut lines = output.split("\n");
        for line in lines {
            if line.trim() == "List of devices attached" {
                continue;
            }
            let mut words = line.trim().split_whitespace();
            let serial = match words.next() {
                None => continue,
                Some(serial) => serial
            };
            match words.next() {
                None => continue,
                Some(device) => {
                    if device != "device" {
                        continue
                    }
                }
            }
            devices.push(serial.to_string())
        }
        Ok(devices)
    }

    pub fn devices(&self) -> Result<Vec<AdbDevice>, RustADBError> {
        let mut devices: Vec<AdbDevice> = vec![];
        let list_devices = match self.list_devices() {
            Ok(list) => list,
            Err(err) => return Err(err)
        };
        for serial in list_devices {
            devices.push(AdbDevice::new(self.adb_path.clone(), serial))
        }
        Ok(devices)
    }

    pub fn device(&self, serial_number: &str) -> Result<AdbDevice, RustADBError> {
        let list_devices = match self.list_devices() {
            Ok(list) => list,
            Err(err) => return Err(err)
        };
        for serial in list_devices {
            if serial_number == serial {
                return Ok(AdbDevice::new(self.adb_path.clone(), serial));
            }
        }
        Err(RustADBError::NotFoundError)
    }

    pub fn connect(&self, serial_number: &str) -> Option<RustADBError> {
        match self.cmd(vec![format!("connect {}", serial_number).as_str()]) {
            Ok(output) => {
                if output.trim() == format!("connected to {}", serial_number) {
                    return None;
                }
                Some(RustADBError::NotFoundError)
            },
            Err(err) => Some(err)
        }
    }
}