use pnet::packet::tcp::{TcpFlags, MutableTcpPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;

use pnet::transport::{
    transport_channel,
    TransportChannelType::Layer4,
    TransportProtocol::Ipv4,
    tcp_packet_iter,
};

use std::net::IpAddr;
use std::time::{Duration, Instant};

pub fn syn_scan(target: IpAddr, port: u16) -> bool {

    // Create transport channel
    let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Tcp));

    let (mut tx, mut rx) =
        transport_channel(4096, protocol)
        .expect("Failed to create channel");

    // Create TCP packet
    let mut buffer = [0u8; 40];

    let mut packet =
        MutableTcpPacket::new(&mut buffer)
        .expect("Failed to create packet");

    packet.set_source(44444);
    packet.set_destination(port);
    packet.set_flags(TcpFlags::SYN);

    // Send SYN
    tx.send_to(packet, target)
        .expect("Failed to send packet");

    // Create iterator
    let mut iter = tcp_packet_iter(&mut rx);

    let start = Instant::now();

    while start.elapsed() < Duration::from_secs(2) {

        match iter.next() {

            Ok((tcp_packet, _)) => {

                if tcp_packet.get_source() == port {

                    // SYN-ACK = port open
                    if tcp_packet.get_flags() & TcpFlags::SYN != 0 &&
                       tcp_packet.get_flags() & TcpFlags::ACK != 0 {

                        return true;
                    }

                    // RST = closed
                    if tcp_packet.get_flags() & TcpFlags::RST != 0 {

                        return false;
                    }
                }
            }

            Err(_) => {}
        }
    }

    false
}
