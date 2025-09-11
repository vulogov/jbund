extern crate log;
use easy_error::{Error};

use bundcore::bundcore::Bund;
use bund_stdlib_text_classifier;

fn init_jbund_stdlib(vm: &mut Bund) -> Result<&mut Bund, Error> {
    log::debug!("Initializing JBUND standard library: {}", &vm.id);
    let _ = bund_stdlib_text_classifier::init_lib(vm);
    Ok(vm)
}

pub fn init_stdlib() {
    let _ = bundcore::add_stdlib("JBUND", init_jbund_stdlib);
}
