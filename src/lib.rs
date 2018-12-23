pub mod configuration;
use async_ssh::Session;
use env_logger;
use futures::Future;
#[macro_use]
extern crate lazy_static;
use log::{debug, info};
use std::collections::HashMap;
use std::error::Error;
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

extern crate serde_derive;

pub fn scan(ips: Vec<String>, settings: configuration::SSHConfig) -> Result<(), Box<dyn Error>> {
    if !SUPPORTED_PACKAGE_MANAGERS.contains_key(&settings.package_manager) {
        unimplemented!("only dpkg is supported")
    }
    for ip in ips {
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

        println!("{}", ::std::str::from_utf8(&data[..]).unwrap());
        debug!("exited with: {}", status);
    }
    Ok(())
}