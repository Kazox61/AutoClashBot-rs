use std::ops::Deref;
use crate::android::Android;
use crate::config::InstanceConfig;
use crate::village::VillageHandler;

pub struct Instance {
    bluestacks_app_path: String,
    bluestacks_conf_path: String,
    bluestacks_shared_folder_path: String,
    bluestacks_instance_name: String,
    minitouch_port: u32,
    instance_index: u16,
    instance_config: InstanceConfig
}

impl Instance {
    pub fn new(bluestacks_app_path: String,
               bluestacks_conf_path: String,
               bluestacks_shared_folder_path: String,
               bluestacks_instance_name: String,
               minitouch_port: u32,
               instance_index: u16,
               instance_config: InstanceConfig
    ) -> Self {

        Instance {
            bluestacks_app_path,
            bluestacks_conf_path,
            bluestacks_shared_folder_path,
            bluestacks_instance_name,
            minitouch_port,
            instance_index,
            instance_config
        }
    }

    pub fn start(&mut self) {
        let mut android = Android::new(
            self.bluestacks_app_path.clone(),
            self.bluestacks_conf_path.clone(),
            self.bluestacks_shared_folder_path.clone(),
            self.bluestacks_instance_name.clone(),
            self.minitouch_port
        );
        android.init();
        let profiles = self.instance_config.profiles.deref().clone();
        let village_handler = VillageHandler::new(android, Vec::from(profiles));
        village_handler.run();
    }
}