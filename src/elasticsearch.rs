use failure::Error;
use log::info;
use std::collections::HashMap;
use std::error::Error as StdError;

#[derive(Debug, Fail)]
enum PushError {
    #[fail(display = "invalid status code : {}", status)]
    InvalidStatusCode { status: i64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    name: String,
    version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    packages: Vec<Package>,
}

pub fn push_scan_results(
    hostname: String,
    packages: HashMap<String, String>,
    settings: crate::configuration::ESConfig,
) -> Result<reqwest::StatusCode, Box<dyn StdError>> {
    let client = reqwest::Client::new();

    let mut vec = Vec::new();

    for (name, version) in packages {
        vec.push(Package {
            name: name,
            version: version,
        });
    }

    let data = Data { packages: vec };

    let url = format!("{}/{}/packages/{}", settings.url, settings.index, hostname).to_owned();
    let res = client.post(&url).json(&data).send()?;

    info!("pushing to {} is {}", url, res.status());
    Ok(res.status())
}

pub fn init_mapping(
    settings: crate::configuration::ESConfig,
) -> Result<reqwest::StatusCode, Box<dyn StdError>> {
    let client = reqwest::Client::new();

    let mapping = r#"{
    "settings": {
        "number_of_replicas": 1
    },
    "mappings": {
        "packages": {
            "properties": {
                "packages": {
                    "type": "nested",
                    "properties": {
                        "name": {
                            "type": "text"
                        },
                        "text": {
                            "type": "text"
                        }
                    }
                }
            }
        }
    }
}"#;

    let url = format!("{}/{}", settings.url, settings.index).to_owned();
    let res = client
        .put(&url)
        .header("Content-Type", "application/json")
        .body(mapping)
        .send()?;
    info!("pushing to {} is {}", url, res.status());
    info!("{:?}", res);
    Ok(res.status())
}
