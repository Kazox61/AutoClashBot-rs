use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::thread;
use std::time::Duration;
use crate::adb::AdbDevice;


const MINITOUCH_SOURCE_PATH: &str = "../files/minitouch/x86/minitouch";
const MINITOUCH_DESTINATION_PATH: &str = "/data/local/tmp";

pub struct Minitouch {
    port: u32,
    default_pressure: u32,
    adb_device: Option<AdbDevice>,
    stream: Option<TcpStream>,
    screen_size_x: u16,
    screen_size_y: u16,
    version: String,
    max_contacts: u8,
    max_x: u32,
    max_y: u32,
    max_pressure: u32,
    pid: u32
}

impl Minitouch {
    pub fn new(port: u32) -> Self {
        Minitouch {
            port,
            default_pressure: 50,
            adb_device: None,
            stream: None,
            screen_size_x: 0,
            screen_size_y: 0,
            version: String::from("0"),
            max_contacts: 0,
            max_x: 0,
            max_y: 0,
            max_pressure: 0,
            pid: 0
        }
    }

    pub fn setup(&mut self, adb_device: AdbDevice, screen_size_x: u16, screen_size_y: u16) {
        self.screen_size_x = screen_size_x;
        self.screen_size_y = screen_size_y;
        self.adb_device = Some(adb_device);
        if !self.has_mini_touch() {
            self.install();
        }
        self.start_server();
    }

    fn has_mini_touch(&self) -> bool {
        let output = self.adb_device.as_ref().unwrap().shell(format!("ls {}", MINITOUCH_DESTINATION_PATH).as_str()).unwrap();
        output.contains("minitouch")
    }

    fn install(&self) {
        self.adb_device.as_ref().unwrap().push(MINITOUCH_SOURCE_PATH, MINITOUCH_DESTINATION_PATH).unwrap();
        self.adb_device.as_ref().unwrap().shell(format!("chmod 755 {}/minitouch", MINITOUCH_DESTINATION_PATH).as_str()).unwrap();
    }

    pub fn start_server(&mut self) {
        let adb_device = self.adb_device.as_ref().unwrap();
        adb_device.forward(
            format!("tcp:{}", self.port).as_str(),
            format!("localabstract:minitouch_{}", self.port).as_str()
        ).unwrap();

        std::process::Command::new("cmd")
            .arg("/C")
            .arg(format!(
                "adb -s {} shell {}/minitouch -n minitouch_{} 2>&1",
                adb_device.serial_number,
                MINITOUCH_DESTINATION_PATH,
                self.port
            ))
            .spawn()
            .unwrap();

        thread::sleep(Duration::from_secs(1));

        let addr: SocketAddr = format!("127.0.0.1:{}", self.port).parse().unwrap();
        match TcpStream::connect_timeout(&addr, Duration::from_secs(1)) {
            Ok(stream) => self.stream = Some(stream),
            Err(_) => panic!("Can't connect to Minitouch")
        }

        let mut header = String::new();
        let mut buffer = [0; 1024];
        let mut stream = self.stream.as_ref().unwrap();
        loop {
            match stream.read(&mut buffer) {
                Ok(_) => {
                    header += String::from_utf8_lossy(&buffer).to_string().as_str();
                    if header.matches("\n").count() >= 3 {
                        break
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                    return;
                }
            }
        }
        let mut rows =header.split("\n");

        let version_row = rows.next().unwrap().to_string();
        self.version = version_row[2..].to_string();

        let split = rows.next().unwrap().to_string();
        let mut device_info = split.split(" ");
        device_info.next().unwrap();
        self.max_contacts= device_info.next().unwrap().parse().unwrap();
        self.max_x = device_info.next().unwrap().parse().unwrap();
        self.max_y = device_info.next().unwrap().parse().unwrap();
        self.max_pressure = device_info.next().unwrap().parse().unwrap();

        let pid_row = rows.next().unwrap().to_string();
        self.pid = pid_row[2..].to_string().parse().unwrap();
        println!("Minitouch Client started with Minitouch-Version: {}, Pid: {}, Max-Contacts: {}, Max-X: {}, Max-Y: {}, Max-Pressure: {}", self.version, self.pid, self.max_contacts, self.max_x, self.max_y, self.max_pressure);
    }

    fn send_command(&self, command: String) {
        self.stream.as_ref().unwrap().write_all(command.as_bytes()).unwrap();
    }

    fn transform(&self, x: u16, y: u16) -> (u32, u32) {
        let tx = x as f64 / self.screen_size_x as f64 * self.max_x as f64;
        let ty = y as f64 / self.screen_size_y as f64 * self.max_y as f64;
        return (tx.round() as u32, ty.round() as u32);
    }

    pub fn click(&self, x: u16, y: u16, duration: u16) {
        let (x, y) = self.transform(x, y);
        println!("{} {}", x, y);
        self.send_command(format!("d 0 {} {} {}\nc\n", x, y, self.default_pressure));
        thread::sleep(Duration::from_secs(duration as u64));
        self.send_command("u 0\nc\n".to_string());
    }
}