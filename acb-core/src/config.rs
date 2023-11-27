use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub use_server: bool,
    pub bluestacks_exe_path: String,
    pub bluestacks_config_path: String,
    pub instance_configs: Vec<InstanceConfig>
}

#[derive(Serialize, Deserialize)]
pub struct InstanceConfig {
    pub instance_name: String,
    pub minitouch_port: u32,
    pub server_port: u32
}