use std::net::{IpAddr, UdpSocket};
use etherparse::PacketBuilder;

const MTU: u16 = 1400;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let sock_addr = socket.local_addr().unwrap();

    let ip = sock_addr.ip();
    let port: u16 = sock_addr.port();

    let ip_octs: [u8; 4] = if let IpAddr::V4(ipv4) = ip {
        ipv4.octets()
    } else {
        println!("Address not IPv4");
        return;
    };
    
    let ttl = 20;
    let builder = PacketBuilder::
        ipv4(ip_octs, ip_octs, ttl)
        .udp(port, port + 1);

    let payload = [1,2,3,4,5,6,7,8];
    let mut result = Vec::<u8>::with_capacity(
        builder.size(payload.len())
        );

    builder.write(&mut result, &payload).unwrap();
    println!("{:?}", result);
    socket.connect("127.0.0.1:8080").expect("connection failed");
    socket.send(&result).expect("couldn't send");
}

