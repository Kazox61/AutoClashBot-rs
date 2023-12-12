use std::process::Output;
use std::str;
use crate::adb::{command, RustADBError};


#[derive(Clone)]
pub struct AdbDevice {
    adb_path: String,
    pub serial_number: String
}

impl AdbDevice {
    pub fn new(adb_path: String, serial_number: String) -> AdbDevice {
        AdbDevice {
            adb_path,
            serial_number
        }
    }

    pub fn start_cmd(&self, command: Vec<&str>) -> Result<Output, RustADBError> {
        command::start_cmd(&self.adb_path, command, Some(self.serial_number.as_str()))
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

    pub fn shell(&self, cmd: &str) -> Result<String, RustADBError> {
        self.cmd(vec!["shell", cmd])
    }

    pub fn root(&self) -> Result<String, RustADBError>{
        self.cmd(vec!["root"])
    }

    pub fn push(&self, local: &str, remote: &str) -> Result<String, RustADBError> {
        self.cmd(vec!["push", local, remote])
    }

    pub fn pull(&self, remote: &str, local: &str) -> Result<String, RustADBError> {
        self.cmd(vec!["pull", remote, local])
    }

    pub fn forward(&self, local: &str, remote: &str) -> Result<String, RustADBError> {
        self.cmd(vec!["forward", local, remote])
    }

    pub fn start_app(&self, package: &str) -> Result<String, RustADBError> {
        self.shell(format!("monkey -p {} 1", package).as_str())
    }

    pub fn stop_app(&self, package: &str) -> Result<String, RustADBError> {
        self.shell(format!("am force-stop {}", package).as_str())
    }
}