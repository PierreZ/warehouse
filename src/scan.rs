use crate::ScanResult;
use async_ssh::Session;
use env_logger;
use futures::Future;
use log::{debug, info};
use std::collections::HashMap;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_io;

lazy_static! {
    static ref SUPPORTED_PACKAGE_MANAGERS: HashMap<String, &'static str> = {
        let mut m = HashMap::new();
        m.insert(
            String::from("dpkg"),
            "dpkg-query -f '${Package}\t${Version}\n' -W",
        );
        m
    };
}

pub fn scan(
    ip: String,
    settings: crate::configuration::SSHConfig,
) -> Result<Vec<ScanResult>, Box<dyn Error>> {
    if !SUPPORTED_PACKAGE_MANAGERS.contains_key(&settings.package_manager) {
        unimplemented!("only dpkg is supported")
    }
    info!("scanning {}", ip);

    // TODO: inject key password
    // let password: Option<&[u8]> = settings.key_password.clone().map(|s| s.as_bytes());
    let key = thrussh_keys::load_secret_key(settings.key_path.clone(), None)?;

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let ls_out = tokio_core::net::TcpStream::connect(&ip.parse().unwrap(), &handle)
        .map_err(thrussh::Error::IO)
        .map_err(thrussh::HandlerError::Error)
        .and_then(|c| Session::new(c, &handle))
        .and_then(|session| session.authenticate_key(&settings.user, key))
        .and_then(|mut session| {
            session.open_exec(
                SUPPORTED_PACKAGE_MANAGERS
                    .get(&settings.package_manager)
                    .unwrap(),
            )
        });

    let channel = core.run(ls_out).unwrap();
    let (channel, data) = core
        .run(tokio_io::io::read_to_end(channel, Vec::new()))
        .unwrap();
    let status = core.run(channel.exit_status()).unwrap();
    debug!("exit status is {}", status);

    let response = ::std::str::from_utf8(&data[..]).unwrap();

    parse_package_manager_response(response, &settings.package_manager, &ip)
}

fn parse_package_manager_response(
    body: &str,
    package_manager: &str,
    ip: &str,
) -> Result<Vec<ScanResult>, Box<dyn Error>> {
    let mut scan_results: Vec<ScanResult> = vec![];

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() as u128 * 1000 + since_the_epoch.subsec_millis() as u128;

    match package_manager {
        "dpkg" => {
            let lines: Vec<&str> = body.lines().collect();
            for line in lines {
                let words: Vec<&str> = line.split("\t").collect();
                if words.len() == 2 {
                    scan_results.push(ScanResult {
                        ip: ip.to_string(),
                        package_manager: package_manager.to_string(),
                        name: words[0].to_string(),
                        version: words[1].to_string(),
                        scan_at: in_ms,
                    });
                }
            }
        }
        &_ => unimplemented!("dpkg only"),
    };

    Ok(scan_results)
}
