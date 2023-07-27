mod builder;
mod connector;
mod stream;

pub use builder::ConnectorBuilder as HttpsConnectorBuilder;
pub use connector::HttpsConnector;
pub use stream::MaybeHttpsStream;

/// The various states of the [`HttpsConnectorBuilder`]
pub mod builderstates {
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub use super::builder::WantsProtocols3;
    pub use super::builder::{WantsProtocols1, WantsProtocols2, WantsSchemes, WantsTlsConfig};
}
