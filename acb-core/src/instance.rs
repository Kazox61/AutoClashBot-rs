use crate::android::Android;
use crate::config::InstanceConfig;

pub struct Instance {
    bluestacks_app_path: String,
    bluestacks_conf_path: String,
    bluestacks_shared_folder_path: String,
    bluestacks_instance_name: String,
    minitouch_port: u32,
    instance_index: u16,
    instance_config: InstanceConfig,
    android: Android
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
        let android = Android::new(
            bluestacks_app_path.clone(),
            bluestacks_conf_path.clone(),
            bluestacks_shared_folder_path.clone(),
            bluestacks_instance_name.clone(),
            minitouch_port
        );

        Instance {
            bluestacks_app_path,
            bluestacks_conf_path,
            bluestacks_shared_folder_path,
            bluestacks_instance_name,
            minitouch_port,
            instance_index,
            instance_config,
            android
        }
    }

    pub fn start(&mut self) {
        self.android.init();
    }
}