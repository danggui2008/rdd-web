use std::{collections::HashMap, fmt::Display, str::FromStr};


use crate::BoxErr;

pub(crate) fn param<T>(params: &HashMap<String, String>, name: &str) -> Result<T, ExtractParamError>
where
    T: FromStr,
    T::Err: Into<BoxErr>,
{
    if let Some(value) = params.get(name) {
        value
            .parse::<T>()
            .map_err(|e| ExtractParamError::InvalidParamValue {
                name: name.to_string(),
                source: e.into(),
            })
    } else {
        Err(ExtractParamError::MissingParamName {
            name: name.to_string(),
        })
    }
}

#[derive(Debug)]
pub enum ExtractParamError {
    MissingParamName { name: String },
    InvalidParamValue { name: String, source: BoxErr },
}

impl Display for ExtractParamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtractParamError::InvalidParamValue { name, source: _ } => {
                write!(f, "invalid param name `{}`", name)
            }
            ExtractParamError::MissingParamName { name } => {
                write!(f, "missing param value:`{}`", name)
            }
        }
    }
}

impl std::error::Error for ExtractParamError {}
