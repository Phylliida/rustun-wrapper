#[macro_use]
extern crate trackable;

use clap::Parser;
use std::thread;
use rusturn::auth::AuthParams;
use trackable::error::MainError;
use fibers_global;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long, default_value_t = 3478)]
    port: u16,
}

fn main() -> Result<(), MainError>{
    let args = Args::parse();
    println!("running stun server on port {}", args.port);
    let addr = track_any_err!(format!("0.0.0.0:{}", args.port).parse())?;
    let server = track!(fibers_global::execute(rustun::server::UdpServer::start(
        fibers_global::handle(),
        addr,
        rustun::server::BindingHandler
    )))?;
    track!(fibers_global::execute(server))?;
    Ok(())
}