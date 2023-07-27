//! # hyper-rustls
//!
//! A pure-Rust HTTPS connector for [hyper](https://hyper.rs), based on
//! [Rustls](https://github.com/rustls/rustls).
//!
//! ## Example client
//!
//! ```no_run
//! # #[cfg(all(feature = "rustls-native-certs", feature = "tokio-runtime", feature = "client", feature = "http1"))]
//! # fn main() {
//! use hyper::{Body, Client, StatusCode, Uri};
//!
//! let mut rt = tokio::runtime::Runtime::new().unwrap();
//! let url = ("https://hyper.rs").parse().unwrap();
//! let https = hyper_rustls::client::HttpsConnectorBuilder::new()
//!     .with_native_roots()
//!     .https_only()
//!     .enable_http1()
//!     .build();
//!
//! let client: Client<_, hyper::Body> = Client::builder().build(https);
//!
//! let res = rt.block_on(client.get(url)).unwrap();
//! assert_eq!(res.status(), StatusCode::OK);
//! # }
//! # #[cfg(not(all(feature = "rustls-native-certs", feature = "tokio-runtime", feature = "client", feature = "http1")))]
//! # fn main() {}
//! ```
//!
//! ## Example server
//!
//! ```no_run
//! # #[cfg(all(feature = "rustls-native-certs", feature = "tokio-runtime", feature = "server"))]
//! # fn main() {
//! use hyper::server::conn::AddrIncoming;
//! use hyper::service::{make_service_fn, service_fn};
//! use hyper::{Body, Method, Request, Response, Server, StatusCode};
//! use hyper_rustls::server::TlsAcceptor;
//! use std::io;
//! use std::fs::File;
//!
//! let mut rt = tokio::runtime::Runtime::new().unwrap();
//! let addr = "127.0.0.1:1337".parse().unwrap();
//!
//! // Load public certificate.
//! let certfile = File::open("examples/sample.pem").unwrap();
//! let mut reader = io::BufReader::new(certfile);
//!
//! // Load and return certificate.
//! let certs = rustls_pemfile::certs(&mut reader).unwrap();
//! let certs = certs.into_iter().map(rustls::Certificate).collect();
//!
//! // Load private key. (see `examples/server.rs`)
//! let keyfile = File::open("examples/sample.rsa").unwrap();
//! let mut reader = io::BufReader::new(keyfile);
//!
//! // Load and return a single private key.
//! let keys = rustls_pemfile::rsa_private_keys(&mut reader).unwrap();
//! let key = rustls::PrivateKey(keys[0].clone());
//!
//! let incoming = AddrIncoming::bind(&addr).unwrap();
//! let acceptor = TlsAcceptor::builder()
//!     .with_single_cert(certs, key).unwrap()
//!     .with_all_versions_alpn()
//!     .with_incoming(incoming);
//! let service = make_service_fn(|_| async { Ok::<_, io::Error>(service_fn(|_req|async {Ok::<_, io::Error>(Response::new(Body::empty()))})) });
//! let server = Server::builder(acceptor).serve(service);
//! // server.await.unwrap();
//! # }
//! # #[cfg(not(all(feature = "rustls-native-certs", feature = "tokio-runtime", feature = "server")))]
//! # fn main() {}
//! ```

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "server")]
#[cfg_attr(docsrs, doc(cfg(feature = "server")))]
/// Items for building a server
pub mod server;

#[cfg(feature = "client")]
#[cfg_attr(docsrs, doc(cfg(feature = "client")))]
/// Items for building a client
pub mod client;

mod config;

#[cfg(feature = "logging")]
mod log {
    pub use log::{debug, trace};
}

#[cfg(not(feature = "logging"))]
mod log {
    macro_rules! trace    ( ($($tt:tt)*) => {{}} );
    macro_rules! debug    ( ($($tt:tt)*) => {{}} );
    pub(crate) use {debug, trace};
}

pub use crate::config::ConfigBuilderExt;
