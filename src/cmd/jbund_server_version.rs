use jsonrpc_http_server::jsonrpc_core::Error;
use serde_json::{Value};

pub fn handler() -> Result<Value, Error> {
    Ok(Value::String(env!("CARGO_PKG_VERSION").into()))
}
