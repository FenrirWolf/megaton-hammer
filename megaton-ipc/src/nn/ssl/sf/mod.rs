mod impl_i_ssl_connection;
pub use self::impl_i_ssl_connection::*;
mod impl_i_ssl_service;
pub use self::impl_i_ssl_service::*;
mod impl_i_ssl_context;
pub use self::impl_i_ssl_context::*;
pub type InternalPki = u32;
pub type SessionCacheMode = u32;
pub type VerifyOption = u32;
pub type RenegotiationMode = u32;
pub type PollEvent = u32;
pub type OptionType = u32;
pub type IoMode = u32;
pub type ContextOption = u32;
pub type CertificateFormat = u32;
pub type SslVersion = u32;
