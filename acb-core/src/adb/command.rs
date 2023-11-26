use std::process::{Command, Output};
use std::str;
use crate::adb::RustADBError;

pub fn start_cmd(adb_path: &str, command: &str, device: Option<&str>) -> Result<Output, RustADBError> {
    let program = match cfg!(target_os = "windows") {
        true => "cmd",
        false => "sh"
    };
    let mut args: Vec<&str> = vec![];

    match cfg!(target_os = "windows") {
        true => args.push("/C"),
        false => {}
    };

    let command = match device {
        None => format!("{} {}", adb_path, command),
        Some(device_serial) => format!("{} -s {} {}", adb_path, device_serial, command)
    };
    args.push(command.as_str());

    match Command::new(program).args(&args).output() {
        Ok(output) => Ok(output),
        Err(_) => Err(RustADBError::CommandError)
    }
    
}