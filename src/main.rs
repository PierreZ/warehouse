use env_logger::{Builder, Env};
use log::*;
use std::path::PathBuf;
use structopt::StructOpt;
use warehouse;

#[derive(StructOpt, PartialEq, Debug, Clone)]
/// handle package inventory on remote servers
struct Opt {
    #[structopt(short = "d", long = "debug", help = "Activate debug mode")]
    debug: bool,

    #[structopt(short = "v", long = "verbose", help = "Activate verbose mode")]
    verbose: bool,

    /// config file
    #[structopt(short = "c", long = "config")]
    config: String,

    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(StructOpt, PartialEq, Debug, Clone)]
enum Cmd {
    #[structopt(name = "scan")]
    /// scan a list of ips
    Scan {
        /// List of IPs to scan
        #[structopt(raw(required = "true", min_values = "1"))]
        ips: Vec<String>,
    },
    #[structopt(name = "inventory")]
    /// provide an Ansible inventory to scan
    Inventory {
        /// Output file
        #[structopt(name = "FILE", parse(from_os_str))]
        inventory: PathBuf,
    },
    #[structopt(name = "init")]
    /// init ES mapping
    Init {},
}

fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Opt::from_args();

    let settings = match warehouse::configuration::Settings::from(args.config) {
        Ok(config) => config,
        Err(error) => panic!("There was a problem opening the config file: {:?}", error),
    };
    debug!("Starting warehouse with {:?}", settings);

    match args.cmd {
        Cmd::Init {} => {
            println!(
                "{:?}",
                warehouse::elasticsearch::init_mapping(settings.elasticsearch.to_owned())
            );
        }
        Cmd::Scan { ips } => {
            for ip in ips {
                let packages = match warehouse::scan::scan(ip.clone(), settings.ssh.clone()) {
                    Ok(p) => p,
                    Err(e) => panic!("There was a problem opening the config file: {:?}", e),
                };
                println!(
                    "{:?}",
                    warehouse::elasticsearch::push_scan_results(
                        packages,
                        settings.elasticsearch.clone()
                    )
                );
            }
        }
        Cmd::Inventory { inventory: _ } => unimplemented!("getting info from Ansible Inventory"),
    }
}
