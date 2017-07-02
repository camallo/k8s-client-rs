//! A pure-Rust asynchronous library for Kubernetes.
//!
//! This library provides support for asynchronous interaction with
//! Kubernetes API server.
//!
//! ## Example
//!
//! ```rust
//! # extern crate futures;
//! # extern crate k8s_client;
//! # extern crate tokio_core;
//! # extern crate serde_yaml;
//! # fn main() {
//! # fn run() -> k8s_client::errors::Result<()> {
//! #
//! # use std::fs::File;
//! # use std::io::BufReader;
//! use futures::Stream;
//! use tokio_core::reactor::Core;
//! use k8s_client::{ClientBuilder, KubeConfig};
//!
//! // Get default cluster context
//! let f = File::open(KubeConfig::default_path())?;
//! let rd = BufReader::new(f);
//! let kcfg: KubeConfig = serde_yaml::from_reader(rd)?;
//! let kctx = kcfg.default_context()?;
//!
//! // Build a basic client
//! let mut tcore = Core::new()?;
//! let kclient = ClientBuilder::try_from((kctx, tcore.handle()))?
//!                              .build_basic();
//!
//! // Check api-server health status
//! let health = tcore.run(kclient.health()?.concat2())?;
//! println!("{}", String::from_utf8_lossy(&health));
//! #
//! # Ok(())
//! # };
//! # }
//! ```

#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
extern crate hyper_rustls;
#[macro_use]
extern crate log;
extern crate rustls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
extern crate tokio_core;

mod builder;
pub use builder::*;

mod client_basic;
pub use client_basic::*;

pub mod errors;

pub mod kubeconfig;
pub use kubeconfig::KubeConfig;

pub mod types;

/// Convenience alias for a future stream of bytes.
pub type StreamBlob = Box<futures::Stream<Item = hyper::Chunk, Error = errors::Error>>;
