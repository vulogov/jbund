extern crate log;
use std::net::SocketAddr;
use crate::cmd::Cli;

use crate::cmd::jbund_server_version;
use crate::cmd::jbund_server_eval;
use crate::cmd::jbund_server_run;

use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;

#[time_graph::instrument]
pub fn run(cli: &Cli) {
    log::debug!("SERVER::run() reached");
    let mut io = IoHandler::default();
    io.add_method("version", |_| {
        jbund_server_version::handler()
	});
    io.add_method("eval", |eval_arg: Params| {
        jbund_server_eval::handler(eval_arg)
	});
    io.add_method("run", |eval_arg: Params| {
        jbund_server_run::handler(eval_arg)
	});

    let addr = match cli.bind.parse::<SocketAddr>() {
        Ok(addr) => addr,
        Err(err) => {
            log::error!("Error parsing bind address: {}", err);
            return;
        }
    };

    let server = match ServerBuilder::new(io)
		.cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Null]))
        .threads(cli.threads)
        .start_http(&addr) {
            Ok(server) => server,
            Err(err) => {
                log::error!("Error startoing server: {}", err);
                return;
            }

    };

    log::debug!("STARTED SERVER at: {}", &cli.bind);

	server.wait();
}
