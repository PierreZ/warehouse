use env_logger;
use log::info;
use std::error::Error;

pub fn scan(ips: Vec<String>) -> Result<(), Box<dyn Error>> {
    for ip in ips {
        info!("scanning {}", ip);
    }
    Ok(())
}
