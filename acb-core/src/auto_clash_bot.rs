use std::fmt::Debug;
use regex::Regex;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use crate::config::Config;
use crate::instance::Instance;
use crate::server::{DaemonCommands, DaemonMessage, Server};


fn get_instances(bluestacks_conf_path: String) -> Vec<String> {
    let contents = fs::read_to_string(bluestacks_conf_path).unwrap();
    let re = Regex::new(r"bst\.instance\.([^.]+)\.abi_list").expect("Failed to create regex");
    let instance_names: Vec<String> = re.captures_iter(&contents)
        .map(|caps| caps[1].to_string())
        .collect();

    instance_names
}

pub struct AutoClashBot {
    conf: Config,
    instance_names: Vec<String>,
    server: Arc<Mutex<Server>>,
    threads: Vec<JoinHandle<()>>,
    sender: Sender<DaemonMessage>,
    receiver: Receiver<DaemonMessage>
}

impl AutoClashBot {
    pub fn new() -> AutoClashBot {
        let config_string = fs::read_to_string("config.json").unwrap();
        let conf: Config = serde_json::from_str(&config_string).unwrap();

        let (tx, rx) = mpsc::channel();

        let server = Arc::new(Mutex::new(Server::new(tx.clone())));

        let instance_names = get_instances(conf.bluestacks_conf_path.clone());

        AutoClashBot {
            conf,
            instance_names,
            server,
            threads: vec![],
            sender: tx,
            receiver: rx
        }
    }

    pub fn start_instance(&mut self, index: u16) {
        let instance_conf = self.conf.instances.get(index as usize).unwrap().clone();
        let instance_name = self.instance_names.get(index as usize).unwrap();

        println!("Start Instance: {}", instance_name);
        let minitouch_port = self.conf.minitouch_start_port + index as u32;

        let mut instance = Instance::new(
            self.conf.bluestacks_app_path.clone(),
            self.conf.bluestacks_conf_path.clone(),
            self.conf.bluestacks_shared_folder_path.clone(),
            instance_name.clone(),
            minitouch_port,
            index,
            instance_conf
        );

        let server_handle = std::thread::spawn(move || instance.start());
        self.threads.push(server_handle);
    }

    pub fn run(&mut self) {
        let server = Arc::clone(&self.server);
        let server_handle = std::thread::spawn(move || server.lock().unwrap().run());
        self.threads.push(server_handle);

        loop {
            let dmsg = self.receiver.recv().unwrap();
            println!("Got: {:?} {} {}", dmsg.daemon_command, dmsg.instance_id.unwrap(), dmsg.message);
            match dmsg.daemon_command {
                DaemonCommands::StartInstance => self.start_instance(dmsg.instance_id.unwrap())
            }
        }
    }
}