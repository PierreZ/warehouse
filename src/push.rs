use failure::Error;
use log::info;
use std::collections::HashMap;
use std::error::Error as StdError;

#[derive(Debug, Fail)]
enum PushError {
    #[fail(display = "invalid status code : {}", status)]
    InvalidStatusCode { status: i64 },
}

pub fn push_scan_results(
    hostname: String,
    packages: HashMap<String, String>,
    settings: crate::configuration::ESConfig,
) -> Result<(), Box<dyn StdError>> {
    let client = reqwest::Client::new();
    info!("{:#?}", packages);
    let url = format!("{}/{}/packages2/{}", settings.url, settings.index, hostname).to_owned();
    let res = client.post(&url).json(&packages).send()?;

    info!("pushing to {} is {}", url, res.status());
    Ok(())
}
