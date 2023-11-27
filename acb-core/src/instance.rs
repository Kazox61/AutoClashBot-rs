use crate::{config::InstanceConfig, server::Client};
use tokio::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Sender, Receiver};

pub struct Instance {
    pub bluestacks_exe_path: String,
    pub bluestacks_config_path: String,
    pub instance_config: InstanceConfig,
    listener: TcpListener,
}

impl Instance {
    pub async fn new(port: u32, bluestacks_exe_path: String, bluestacks_config_path: String, instance_config: InstanceConfig) -> Instance {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();
        Instance {
            bluestacks_exe_path,
            bluestacks_config_path,
            instance_config,
            listener
        }
    }


    pub async fn run(&mut self) {
        let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
        self.handle_clients(sender).await;
    }

    pub async fn handle_clients(&self, sender: Sender<String>) {
        while let Ok((stream, _)) = self.listener.accept().await {
            let sender = sender.clone();
            let mut client = Client::new(sender, stream);
            tokio::spawn(async move {
                let received = client.read_all().await;
                println!("Received: {}", received);
            });
        }
    }
}