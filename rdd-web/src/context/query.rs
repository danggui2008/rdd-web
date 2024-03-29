use std::fmt::Display;

use hyper::{Body, Request};
use serde::Deserialize;

pub(crate) fn query<'de, T>(req: &'de Request<Body>) -> Result<T, ExtractQueryError>
where
    T: Deserialize<'de>,
{
    let query = req.uri().query().unwrap_or_default();
    serde_urlencoded::from_str(query).map_err(|e| ExtractQueryError(e))
}
#[derive(Debug)]
pub struct ExtractQueryError(pub serde::de::value::Error);

impl Display for ExtractQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to deserialize query string({})", &self.0)
    }
}

impl std::error::Error for ExtractQueryError {}
