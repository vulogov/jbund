use jsonrpc_http_server::jsonrpc_core::{Params, Error};
use serde_json::{Value};
use bundcore::bundcore::Bund;
use rust_dynamic::types::*;

pub fn handler(eval_args: Params) -> Result<Value, Error> {
    let scripts = match eval_args {
        Params::None => {
            log::debug!("BUND::EVAL handler did not received any parameters.");
            return Err(Error::invalid_params(format!("BUND::EVAL handler did not received any parameters.")));
        }
        Params::Array(data) => {
            let mut res: Vec<String> = Vec::new();
            for i in data {
                match i {
                    Value::String(s_data) => {
                        res.push(s_data.to_string());
                    }
                    _ => continue,
                }
            }
            res
        }
        Params::Map(_data) => {
            log::debug!("BUND::EVAL received unsupported parameter type.");
            return Err(Error::invalid_params(format!("BUND::EVAL received unsupported parameter type.")));
        }
    };
    let mut bund = Bund::new();
    for s in scripts {
        match bund.eval(s) {
            Ok(_) => continue,
            Err(err) => {
                return Err(Error::invalid_params(format!("{}", err)));
            }
        }
    }
    let val = match bund.vm.stack.pull() {
        Some(val) => {
            match val.cast_value_to_json() {
                Ok(jvalue) => jvalue,
                Err(_) => {
                    match val.conv(STRING) {
                        Ok(svalue) => {
                            Value::String(svalue.cast_string().unwrap())
                        }
                        Err(err) => {
                            return Err(Error::invalid_params(format!("{}", err)));
                        }
                    }
                }
            }
        }
        None => {
            return Err(Error::invalid_params(format!("BUND interpreter stack is empty")));
        }
    };
    drop(bund);
    Ok(val.into())
}
