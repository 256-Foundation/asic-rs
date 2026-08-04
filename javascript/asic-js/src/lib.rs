mod factory;
mod miner;

use napi::{Error, Result, Status};

fn to_napi_error(error: impl std::fmt::Display) -> Error {
    Error::new(Status::GenericFailure, error.to_string())
}

fn to_js_value<T: serde::Serialize>(value: T) -> Result<serde_json::Value> {
    serde_json::to_value(value).map_err(to_napi_error)
}
