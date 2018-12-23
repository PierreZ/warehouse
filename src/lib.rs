use async_ssh::Session;
use env_logger;
use futures::Future;
use log::info;
use std::error::Error;
use tokio_io;

pub fn scan(ips: Vec<String>) -> Result<(), Box<dyn Error>> {
    for ip in ips {
        info!("scanning {}", ip);
        let key = thrussh_keys::load_secret_key("/home/pierrez/.ssh/id_ed25519", None)?;

        info!("key OK");

        let mut core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();
        let ls_out = tokio_core::net::TcpStream::connect(&ip.parse().unwrap(), &handle)
            .map_err(thrussh::Error::IO)
            .map_err(thrussh::HandlerError::Error)
            .and_then(|c| Session::new(c, &handle))
            .and_then(|session| session.authenticate_key("debian", key))
            .and_then(|mut session| {
                session.open_exec("dpkg-query -f '${Package}\t${Version}\n' -W")
            });

        let channel = core.run(ls_out).unwrap();
        let (channel, data) = core
            .run(tokio_io::io::read_to_end(channel, Vec::new()))
            .unwrap();
        let status = core.run(channel.exit_status()).unwrap();

        println!("{}", ::std::str::from_utf8(&data[..]).unwrap());
        println!("exited with: {}", status);
    }
    Ok(())
}
