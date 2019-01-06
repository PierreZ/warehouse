use crate::ScanResult;
use log::info;
use std::error::Error as StdError;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ActionAndMetadata {
    index: IndexMetadata,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IndexMetadata {
    #[serde(rename = "_index")]
    index: String,
    #[serde(rename = "_type")]
    estype: String,
    #[serde(rename = "_id")]
    id: String,
}

fn create_bulk_body(
    scan_results: Vec<ScanResult>,
    settings: &crate::configuration::ESConfig,
) -> Result<String, Box<dyn StdError>> {
    let mut body = String::new();

    for scan_result in scan_results {
        let index = IndexMetadata {
            index: settings.index.to_owned(),
            estype: "packages".to_string(),
            id: scan_result.get_id(),
        };

        let index_json = serde_json::to_string(&ActionAndMetadata { index: index })?;
        body += &index_json;
        body += "\n";
        let scan_json = serde_json::to_string(&scan_result)?;
        body += &scan_json;
        body += "\n";
    }

    Ok(body)
}

pub fn push_scan_results(
    scan_results: Vec<ScanResult>,
    settings: crate::configuration::ESConfig,
) -> Result<(), Box<dyn StdError>> {
    let body = create_bulk_body(scan_results, &settings)?;
    let client = reqwest::Client::new();

    let url = format!("{}/_bulk", settings.url).to_owned();
    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()?;

    info!("pushing to {} is {}", url, res.status());
    Ok(())
}

pub fn init_mapping(
    settings: crate::configuration::ESConfig,
) -> Result<reqwest::StatusCode, Box<dyn StdError>> {
    let client = reqwest::Client::new();

    let mapping = r#"{
    "settings": {
        "number_of_replicas": 1
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
