use crate::os;
use crate::magic_packet::MagicPacket;
use std::net::UdpSocket;
use std::net::Ipv4Addr;

pub struct Server {
    ip: Ipv4Addr,
    port: u16,
}

pub fn create(ip: Ipv4Addr, port: u16) -> Server {
    Server{ip, port}
}

impl Server {
    pub fn wait_for(self: &Self, packet: MagicPacket) -> std::io::Result<()> {
        let socket = UdpSocket::bind((self.ip, self.port))?;
        let mut received_magic_packet = packet.create_blank();

        loop {
            println!("waiting for magic packet: {:?}", packet.as_bytes());

            let (_bytes_read, _src) = socket.recv_from(&mut received_magic_packet.as_mut_slice())?;
            println!("read: {:?}", received_magic_packet.as_bytes());

            let is_valid = packet.eq(&received_magic_packet);

            received_magic_packet.clear();
            println!("clean: {:?}", received_magic_packet.as_bytes());

            if is_valid { self.handle_valid_packet() }
        }
    }

    fn handle_valid_packet(self: &Self) {
        println!("triggering sleep");
        match os::trigger_sleep() {
            Ok(_) => { println!("sucessfully slept" ); }
            Err(e) => { println!("{:?}", e) }
        }
    }
}
