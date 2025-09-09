extern crate log;

pub mod common;
pub mod setloglevel;
pub mod jbund_server;
pub mod jbund_server_version;
pub mod jbund_server_eval;
pub mod jbund_server_run;
pub mod jbund_adam;

use lazy_static::lazy_static;
use std::sync::Mutex;
use clap::{Parser};
use time_graph;

lazy_static! {
    pub static ref CLI: Mutex<Cli> = {
        let e: Mutex<Cli> = Mutex::new(Cli::parse());
        e
    };
}

fn do_panic() {
    log::debug!("Setting a global panic handler");
    better_panic::Settings::auto()
        .most_recent_first(false)
        .lineno_suffix(true)
        .verbosity(better_panic::Verbosity::Full)
        .install();
}

pub fn main() {
    let cli = Cli::parse();
    setloglevel::setloglevel(&cli);
    do_panic();
    let init_cli = CLI.lock().unwrap();
    log::debug!("Initialize global CLI");
    drop(init_cli);
    log::debug!("JBUND context initialized ...");

    if cli.profile {
        log::debug!("Enable JBUND profiler");
        time_graph::enable_data_collection(true);
    }
    jbund_adam::run(&cli);
    jbund_server::run(&cli);
    if cli.profile {
        log::debug!("Generating JBUND profiler report");
        let graph = time_graph::get_full_graph();
        println!("{}", graph.as_table());
    }
}

#[derive(Parser, Clone, Debug)]
#[clap(name = "jbund")]
#[clap(author = "Vladimir Ulogov <vladimir@ulogov.us>")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "JBUND SERVICE", long_about = "JSON-RPC interface for BUND language platform")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Execute internal profiler")]
    pub profile: bool,

    #[clap(short, long, value_delimiter = ' ', num_args = 0.., help="List of BUND sripts executed at bootstrap")]
    pub bootstrap: Option<Vec<String>>,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Disable colors in output")]
    pub nocolor: bool,

    #[clap(help="Set the initial stack name", long)]
    pub stack: Option<String>,

    #[clap(help="Bind JSONRPC server to the host and port", long, default_value_t = String::from("127.0.0.1:10099".to_string()))]
    pub bind: String,

    #[arg(long, help="The number of server threads", default_value_t = 3)]
    threads: usize,
}
