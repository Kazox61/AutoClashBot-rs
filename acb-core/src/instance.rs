use crate::{config::InstanceConfig, server::Client};
use std::sync::mpsc::{self, Sender, Receiver};
use crate::EVENT_EMITTER;
use tokio::time::{sleep, Duration};

pub struct Instance {
    pub bluestacks_exe_path: String,
    pub bluestacks_config_path: String,
    pub instance_config: InstanceConfig,
}

impl Instance {
    pub async fn new(port: u32, bluestacks_exe_path: String, bluestacks_config_path: String, instance_config: InstanceConfig) -> Instance {
        Instance {
            bluestacks_exe_path,
            bluestacks_config_path,
            instance_config
        }
    }

    pub async fn work1(&self) {

    }

    pub async fn work2(&self) {

    }

    pub async fn run(&mut self) {
        EVENT_EMITTER.lock().await.on("Hello", |text: String| async { println!("{}", self.bluestacks_config_path)});
        //use the above line insteader of the one below
        //EVENT_EMITTER.lock().await.on("Hello", |text: String| async { println!("Hello World")});
        loop {
            sleep(Duration::from_secs(5)).await;
            println!("Wait 5 secs");
            
        }
    }
}