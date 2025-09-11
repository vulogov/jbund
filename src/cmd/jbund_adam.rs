extern crate log;
use lazy_static::lazy_static;
use std::sync::Mutex;

use bundcore::bundcore::Bund;
use bund_stdlib_text_classifier;

use crate::cmd::Cli;
use crate::cmd::common;
use crate::stdlib;

lazy_static! {
    pub static ref ADAM: Mutex<Bund> = {
        let a: Mutex<Bund> = Mutex::new(Bund::new());
        a
    };
}

pub fn run(cli: &Cli) {
    log::debug!("Setting up STDLIB");
    stdlib::init_stdlib();
    log::debug!("Creating an ADAM");
    let mut bund = ADAM.lock().unwrap();
    log::debug!("Initializing ADAM instance of BUND");
    let _ = bund_stdlib_text_classifier::init_lib(&mut *bund as &mut bundcore::bundcore::Bund);
    match &cli.bootstrap {
        Some(bootstrap) => {
            for s in bootstrap {
                match common::get_file_from_relative_file(s.to_string()) {
                    Some(script) => {
                        log::debug!("Bootstrap script found: {}", &s);
                        match bund.eval(script) {
                            Ok(_) => {}
                            Err(err) => {
                                log::error!("Error executing bootstrap: {}", err);
                            }
                        }
                    }
                    None => {
                        log::error!("Bootstrap script not found: {}", &s);
                    }
                }
            }
        }
        None => {
            log::debug!("No bootrtrap scripts were passed");
        }
    }
    drop(bund);
}
