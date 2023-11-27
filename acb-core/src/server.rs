use tokio::net::{TcpListener, TcpStream};
use tokio::io::{BufReader, AsyncBufReadExt, AsyncReadExt};
use std::io::{Read, Write};

use std::sync::mpsc::{self, Sender, Receiver};

pub struct Client {
    sender: Sender<String>,
    buff_reader: BufReader<TcpStream>
}

impl Client {
    pub fn new(sender: Sender<String>, stream: TcpStream) -> Client {
        let mut buff_reader = BufReader::new(stream);
        Client { sender, buff_reader }
    }

    pub async fn read(&mut self) {
        let mut buffer = [0, 255];
        let amount = self.buff_reader.read(&mut buffer).await.unwrap();
        let received = String::from_utf8_lossy(&buffer[..amount]).to_string();
        println!("{}", received);
    }

    pub async fn read_all(&mut self) -> String {
        let mut result = String::new();

        loop {
            let mut buffer = [0; 255];

            match self.buff_reader.read(&mut buffer).await {
                Ok(0) => break,
                Ok(amount) => {
                    let received = String::from_utf8_lossy(&buffer[..amount]).to_string();
                    result.push_str(&received);
                    if result.as_bytes().last() == Some(&b'\0') {
                        break;
                    }
                }
                Err(_) => break
            }
        }
        result
    }
}
