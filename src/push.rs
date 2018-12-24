use std::collections::HashMap;
use std::error::Error;

pub fn push_scan_results(
    hostname: String,
    packages: HashMap<String, String>,
    settings: crate::configuration::ESConfig,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
