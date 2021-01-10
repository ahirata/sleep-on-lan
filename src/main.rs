mod magic_packet;
mod server;
mod os;
mod mac;
use mac::{Mac, mac_from_str};
use magic_packet::MagicPacket;
use structopt::StructOpt;
use std::net::Ipv4Addr;


/// Put the computer to sleep upon receiving the reversed mac on the magic packet
#[derive(StructOpt)]
struct Cli {
    /// The ip to bind
    ip: Ipv4Addr,
    /// The port to bind
    port: u16,
    /// The mac that will be reversed and match against the received magic packet
    #[structopt(parse(try_from_str = mac_from_str))]
    mac: Mac
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    server::create(args.ip, args.port).wait_for(MagicPacket::from(args.mac))
}
