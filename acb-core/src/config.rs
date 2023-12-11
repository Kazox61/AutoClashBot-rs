use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub bluestacks_app_path: String,
    pub bluestacks_conf_path: String,
    pub bluestacks_instance_conf_path: String,
    pub bluestacks_shared_folder_path: String,
    pub minitouch_start_port: u32,
    pub instances: Vec<InstanceConfig>
}

#[derive(Serialize, Deserialize)]
pub struct InstanceConfig {
    pub profiles: Vec<String>
}

impl Clone for InstanceConfig {
    fn clone(&self) -> Self {
        InstanceConfig {
            profiles: self.profiles.clone()
        }
    }
}