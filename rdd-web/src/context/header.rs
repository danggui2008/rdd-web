use std::{fmt::Display, str::FromStr};

use hyper::{Body, Request};

use crate::BoxErr;

pub fn header<T>(req: &Request<Body>, name: &str) -> Result<T, ExtractHeaderError>
where
    T: FromStr,
    T::Err: Into<BoxErr>,
{
    if let Some(value) = req.headers().get(name) {
        match value.to_str() {
            Ok(s) => s
                .parse::<T>()
                .map_err(|e| ExtractHeaderError::InvalidHeader {
                    name: name.to_string(),
                    source: e.into(),
                }),
            Err(e) => Err(ExtractHeaderError::InvalidHeader {
                name: name.to_string(),
                source: e.into(),
            }),
        }
    } else {
        Err(ExtractHeaderError::MissingHeader {
            name: name.to_string(),
        })
    }
}

#[derive(Debug)]
pub enum ExtractHeaderError {
    MissingHeader { name: String },
    InvalidHeader { name: String, source: BoxErr },
}

impl Display for ExtractHeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtractHeaderError::InvalidHeader { name, source: _ } => {
                write!(f, "invalid request header `{}`", name)
            }
            ExtractHeaderError::MissingHeader { name } => {
                write!(f, "missing request header:`{}`", name)
            }
        }
    }
}

impl std::error::Error for ExtractHeaderError {}
