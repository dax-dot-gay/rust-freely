//! Asynchronous wrapper for the WriteFreely/Write.as API, as the [existing library](https://docs.rs/writefreely_client/latest/writefreely_client/) is no longer maintained
//! 
//! Provides an implementation of the WriteFreely [API](https://developers.write.as/docs/api/). Currently, this library supports the following API features:
//!  - Token & Username/Password authentication
//!  - Most post management endpoints
//!  - All collection endpoints
//!  - All user endpoints except channels

#![warn(missing_docs)]
mod client;
pub use client::api_client;
pub use client::api_models;
pub use client::api_wrapper;
pub use client::api_handlers;
