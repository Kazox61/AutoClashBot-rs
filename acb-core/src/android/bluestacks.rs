use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::process::Command;
use std::ptr::write;
use std::thread;
use std::time::Duration;
use regex::Regex;
use crate::adb::{AdbClient, AdbDevice};

pub struct Bluestacks {
    pub app_path: String,
    pub conf_path: String,
    pub shared_folder_path: String,
    pub instance_name: String,
    pub adb_port: u32,
    pub adb_client: AdbClient,
    pub adb_device: Option<AdbDevice>
}

impl Bluestacks {
    pub fn new(
        app_path: String,
        conf_path: String,
        shared_folder_path: String,
        instance_name: String
    ) -> Self {
        let adb_client = AdbClient::new();

        Bluestacks {
            app_path,
            conf_path,
            shared_folder_path,
            instance_name,
            adb_port: 0,
            adb_client,
            adb_device: None
        }
    }

    pub fn start(&mut self) -> (AdbClient, AdbDevice) {
        self.setup();
        self.start_device();

        thread::sleep(Duration::from_secs(10));

        let adb_port = self.get_config_value("status.adb_port");
        self.adb_port = adb_port.parse().unwrap();
        println!("Abd-port: {}", self.adb_port);

        let serial_number = format!("localhost:{}", self.adb_port);
        let serial_number = serial_number.as_str();
        self.adb_client.connect(serial_number);

        let adb_device = self.adb_client.device(serial_number).unwrap();
        self.adb_device = Some(adb_device.clone());
        adb_device.root().unwrap();
        thread::sleep(Duration::from_secs(5));
        return (self.adb_client.clone(), adb_device.clone())
    }

    fn setup(&self) {
        let display_name = format!("acb-{}", self.instance_name);
        let values: Vec<(&str, &str)> = vec![
            ("enable_root_access", "1"),
            ("fb_width", "800"),
            ("fb_height", "600"),
            ("dpi", "240"),
            ("show_sidebar", "0"),
            ("display_name", display_name.as_str()),
            ("enable_fps_display", "1"),
            ("google_login_popup_shown", "0")
        ];
        self.set_instance_config_values(values)
    }

    fn set_instance_config_values(&self, config_values: Vec<(&str, &str)>) {
        let mut file = OpenOptions::new().read(true).write(true).open(&self.conf_path).unwrap();
        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();

        for (key, value) in config_values {
            let pattern = format!(
                "bst.instance.{}.{}=\"([^.]*)\"",
                regex::escape(&self.instance_name),
                regex::escape(key)
            );
            let regex = Regex::new(&pattern).unwrap();
            text = regex.replace(&text, |caps: &regex::Captures| {
                format!("bst.instance.{}.{}=\"{}\"", &self.instance_name, key, value)
            }).to_string();
        }

        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        file.set_len(text.len() as u64).unwrap();
        file.write_all(text.as_bytes()).unwrap();
    }

    fn get_config_value(&self, key: &str) -> String {
        let mut file = OpenOptions::new().read(true).open(&self.conf_path).unwrap();
        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();
        let pattern = format!(
            "bst.instance.{}.{}=\"([^.]*)\"",
            regex::escape(&self.instance_name),
            regex::escape(key)
        );
        let regex = Regex::new(&pattern).ok().unwrap();

        regex.captures(&text).map(|captures| captures[1].to_string()).unwrap()
    }

    fn start_device(&self) {
        Command::new(&self.app_path)
            .arg("--instance")
            .arg(&self.instance_name)
            .spawn()
            .unwrap();
    }

    pub fn get_screen_size(&self) -> (u16, u16){
        let output = self.adb_device.as_ref().unwrap().shell("dumpsys window | grep cur= |tr -s \" \" | cut -d \" \" -f 4|cut -d \"=\" -f 2").unwrap();
        println!("SS: {}", output);
        let output = output.replace("\n", "");
        let values = output.trim().split_once("x").unwrap();
        return (values.0.parse().unwrap(), values.1.parse().unwrap())
    }
}