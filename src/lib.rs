#[cfg(feature = "default")]
pub extern crate tokio;

#[cfg(feature = "default")]
pub use tokio::prelude::*;
#[cfg(feature = "default")]
pub use tokio::runtime::Runtime;

pub mod constants;
pub mod types;

#[cfg(feature = "default")]
use futures::prelude::*;
use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use std::rc::Rc;


/// Reference counted Hyper client with custom https connector
pub type HttpsClient = Rc<Client<HttpsConnector<HttpConnector>, hyper::Body>>;
