use jsonrpc_http_server::jsonrpc_core::{Params, Error};
use serde_json::{Value};
// use bundcore::bundcore::Bund;
// use rust_dynamic::types::*;

pub fn handler(run_args: Params) -> Result<Value, Error> {
    let _run_params = match run_args {
        Params::None => {
            log::debug!("BUND::RUN handler did not received any parameters.");
            return Err(Error::invalid_params(format!("BUND::RUN handler did not received any parameters.")));
        }
        Params::Array(_) => {
            log::debug!("BUND::RUN handler did not received any parameters.");
            return Err(Error::invalid_params(format!("BUND::RUN handler did not received any parameters.")));
        }
        Params::Map(_data) => {
            log::debug!("BUND::EVAL received unsupported parameter type.");
            return Err(Error::invalid_params(format!("BUND::RUN received unsupported parameter type.")));
        }
    };
    // log::debug!("BUNDCORE version: {}", bundcore::version());
    // let mut bund = Bund::new();
    //
    // drop(bund);
    // Ok("".to_string().into())
}
