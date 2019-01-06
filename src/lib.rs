#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

pub mod configuration;
pub mod elasticsearch;
pub mod scan;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    ip: String,
    package_manager: String,
    name: String,
    version: String,
    scan_at: u128,
}

impl ScanResult {
    pub fn get_id(&self) -> String {
        format!("{}|{}", self.ip, self.name)
    }
}
