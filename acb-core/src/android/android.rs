use crate::adb::{AdbClient, AdbDevice};
use crate::android::{Bluestacks, Minitouch};

pub struct Android {
    bluestacks_app_path: String,
    bluestacks_conf_path: String,
    bluestacks_shared_folder_path: String,
    bluestacks_instance_name: String,
    minitouch_port: u32,
    bluestacks: Bluestacks,
    minitouch: Minitouch,
    adb_client: Option<AdbClient>,
    adb_device: Option<AdbDevice>
}

impl Android {
    pub fn new(
        bluestacks_app_path: String,
        bluestacks_conf_path: String,
        bluestacks_shared_folder_path: String,
        bluestacks_instance_name: String,
        minitouch_port: u32,
    ) -> Self {
        let bluestacks = Bluestacks::new(
            bluestacks_app_path.clone(),
            bluestacks_conf_path.clone(),
            bluestacks_shared_folder_path.clone(),
            bluestacks_instance_name.clone()
        );
        let minitouch = Minitouch::new(minitouch_port);

        Android {
            bluestacks_app_path,
            bluestacks_conf_path,
            bluestacks_shared_folder_path,
            bluestacks_instance_name,
            minitouch_port,
            bluestacks,
            minitouch,
            adb_client: None,
            adb_device: None
        }
    }

    pub fn init(&mut self) {
        let (client, device) = self.bluestacks.start();
        let screen_size = self.bluestacks.get_screen_size();
        println!("Screensize: {} {}", screen_size.0, screen_size.1);
        self.minitouch.setup(device.clone(), screen_size.0, screen_size.1);
        self.adb_client = Some(client);
        self.adb_device = Some(device);
        println!("Click test");
        self.minitouch.click(400, 300, 10);
    }
}