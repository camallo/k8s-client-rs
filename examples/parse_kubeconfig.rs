extern crate error_chain;
extern crate k8s_client;
extern crate serde;
extern crate serde_yaml;

use error_chain::ChainedError;
use k8s_client::errors::*;
use k8s_client::KubeConfig;
use std::{fs, io};

fn main() {
    if let Err(e) = run() {
        print!("{}", e.display());
        std::process::exit(1);
    };
}

fn run() -> Result<()> {
    let f = fs::File::open(KubeConfig::default_path())?;
    let rd = io::BufReader::new(f);
    let cfg: KubeConfig = serde_yaml::from_reader(rd)?;

    println!("Configured clusters:");
    for c in &cfg.clusters {
        println!("* {}", c.name);
    }
    println!("");

    println!("Current default context:");
    println!("{:?}", cfg.default_context()?);

    Ok(())
}
