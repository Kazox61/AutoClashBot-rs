use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
#[repr(u16)]
pub enum DaemonCommands {
    StartInstance = 0
}

impl DaemonCommands {
    fn from_u16(value: u16) -> Option<DaemonCommands> {
        match value {
            0 => Some(DaemonCommands::StartInstance),
            _ => None
        }
    }
}

pub struct DaemonMessage {
    pub daemon_command: DaemonCommands,
    pub instance_id: Option<u16>,
    pub message: String
}

pub struct Server {
    listener: TcpListener,
    sender: Sender<DaemonMessage>
}

impl Server {
    pub fn new(sender: Sender<DaemonMessage>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:9339").unwrap();
        println!("Server started");
        Server {
            listener,
            sender
        }
    }

    pub fn run(&self) {
        println!("Server waiting for incoming connections");
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let sender = self.sender.clone();
                    std::thread::spawn(move || {
                        println!("New client connected");
                        let mut client = Client::new(stream, sender);
                        client.read()
                    });
                },
                Err(e) => eprintln!("Error accepting connection: {}", e)
            }
        }
    }
}


pub struct Client {
    stream: TcpStream,
    sender: Sender<DaemonMessage>
}

impl Client {
    pub fn new(stream: TcpStream, sender: Sender<DaemonMessage>) -> Client {
        Client { stream, sender }
    }

    pub fn read(&mut self) {
        let mut buffer = [0; 9];
        loop {
            match self.stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        println!("Client disconnected: {:?}", self.stream.peer_addr());
                        return;
                    }

                    let instance_id = std::io::Cursor::new(&buffer[..2]).read_u16::<BigEndian>().unwrap();
                    let command_id = std::io::Cursor::new(&buffer[2..4]).read_u16::<BigEndian>().unwrap();

                    let dmsg = DaemonMessage {
                        daemon_command: DaemonCommands::from_u16(command_id).unwrap(),
                        instance_id: Some(instance_id),
                        message: String::new()
                    };


                    self.sender.send(dmsg).unwrap();
                    //println!("Received: {}", String::from_utf8_lossy(&buffer[0..bytes_read]));
                }
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                    return;
                }
            }
        }
    }
}
