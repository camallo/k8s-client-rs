extern crate error_chain;
extern crate futures;
extern crate k8s_client;
extern crate hyper;
extern crate hyper_rustls;
extern crate rustls;
extern crate serde_yaml;
extern crate serde_json;
extern crate tokio_core;

use error_chain::ChainedError;
use futures::Stream;
use tokio_core::reactor;
use k8s_client::errors::*;
use k8s_client::{ClientBuilder, KubeConfig};
use k8s_client::types::VersionInfo;
use std::{fs, io};

fn main() {
    if let Err(e) = run() {
        print!("{}", e.display_chain());
        std::process::exit(1);
    };
}

fn run() -> Result<()> {
    let mut tcore = reactor::Core::new().expect("failed to create core");
    let handle = tcore.handle();

    let f = fs::File::open(KubeConfig::default_path())?;
    let rd = io::BufReader::new(f);
    let cfg: KubeConfig = serde_yaml::from_reader(rd)?;
    let cur_ctx = cfg.default_context()?;

    let k8s_cfg = ClientBuilder::try_from((cur_ctx.clone(), handle.clone()))?;
    let client = k8s_cfg.build_basic();
    let fut_ver = client.version().chain_err(|| "failed to build request")?;
    let ver_body = tcore.run(fut_ver.concat2()).chain_err(
        || "failed to get cluster version",
    )?;
    let ver: VersionInfo = serde_json::from_slice(ver_body.as_ref())?;

    println!(
        r#"k8s cluster at {}, version: "{}""#,
        cur_ctx.cluster.server,
        ver
    );
    Ok(())
}
