use std::process::{Command, Output};
use std::str;
use crate::adb::RustADBError;

pub fn start_cmd(adb_path: &str, mut args: Vec<&str>, device: Option<&str>) -> Result<Output, RustADBError> {
    let mut cmd_args: Vec<String> = args.into_iter().map(String::from).collect();
    match device {
        None => (),
        Some(device_serial) => {
            cmd_args.insert(0, device_serial.to_string());
            cmd_args.insert(0, "-s".to_string());
        }
    };

    match Command::new(adb_path).args(&cmd_args).output() {
        Ok(output) => Ok(output),
        Err(_) => Err(RustADBError::CommandError)
    }
}