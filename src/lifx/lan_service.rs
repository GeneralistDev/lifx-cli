use std::{net::{UdpSocket, SocketAddr}, error::Error};

use log::debug;

use super::lan::{LifxPacket, BinarySerializable, self};

pub struct LanService {
    sequence: u8,
}

impl LanService {
    pub fn new() -> LanService {
        LanService { sequence: 0 }
    }

    /*
        Send a UDP query to all devices (broadcast)
    */
    pub fn broadcast_query(&self, packet_type: LifxPacket, query: Option<Box<dyn BinarySerializable>>) -> Result<(Vec<u8>, SocketAddr), Box<dyn Error>> {
        let header = lan::Header::new(self.sequence, packet_type as u16, None);

        let mut encoded_header: Vec<u8> = bincode::serialize(&header).unwrap();

        if let Some(query) = query {
            let mut encoded_payload: Vec<u8> = query.serialize();

            encoded_header.append(&mut encoded_payload);
        }

        let socket = UdpSocket::bind("0.0.0.0:56701").expect("Could not open UDP socket");
        socket.set_broadcast(true).expect("could not set socket to broadcast");
        socket.send_to(&encoded_header.as_slice(), "255.255.255.255:56700").expect("failed to send message");

        let mut buffer: [u8; 1024] = [0; 1024];

        let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("no data received");

        debug!("{:?}", number_of_bytes);
        debug!("{:?}", src_addr);

        let mut res_data: Vec<u8> = vec![0; number_of_bytes];

        for i in 0..number_of_bytes {
            res_data[i] = buffer[i];
        }

        Ok((res_data, src_addr))
    }

    /*
        Send a UDP command to a specific device
    */
    pub fn send_command(&self, ip: SocketAddr, packet_type: LifxPacket, command: Box<dyn BinarySerializable>) {
        let header = lan::Header::new(self.sequence, packet_type as u16, None);

        let mut encoded_header: Vec<u8> = bincode::serialize(&header).unwrap();

        let mut encoded_payload: Vec<u8> = command.serialize();

        encoded_header.append(&mut encoded_payload);

        let socket = UdpSocket::bind("0.0.0.0:56701").expect("Could not open UDP socket");
        socket.send_to::<SocketAddr>(&encoded_header.as_slice(), ip).expect("failed to send message");
    }
}