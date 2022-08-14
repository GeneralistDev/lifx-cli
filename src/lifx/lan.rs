use std::net::{IpAddr, SocketAddr};

use mac_address::get_mac_address;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Header {
    /* frame */
    pub size: u16,
    pub protocol_addressable_tagged_origin: u16,
    pub source: u32,

    /* frame address */
    pub target: [u8; 8],
    pub reserved1: [u8; 6],
    pub res_required_ack_required: u8,
    pub sequence: u8,
    
    /* protocol header */
    pub reserved3: u64,
    pub packet_type: u16,
    pub reserved4: u16,

    /* variable length payload follows */
}

impl Header {
    pub fn new(sequence: u8, packet_type: u16) -> Header {
        // let mac = match get_mac_address() {
        //     Ok(Some(ma)) => {
        //         println!("MAC addr = {}", ma);
        //         println!("bytes = {:?}", ma.bytes());
        //         ma.bytes()
        //     }
        //     Ok(None) => panic!("No MAC address found."),
        //     Err(e) => panic!("{:?}", e),
        // };

        let target: [u8; 8] = [0; 8];
        let reserved1: [u8; 6] = [0; 6];

        // for i in 0..mac.len() {
        //     target[i] = mac[i];
        // }

        Header { 
            size: 49,
            protocol_addressable_tagged_origin: 1024 | 0b00010000_00000000,
            source: 2,
            target: target,
            reserved1: reserved1,
            res_required_ack_required: 2,
            sequence: sequence,
            reserved3: 0,
            packet_type: packet_type,
            reserved4: 0,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SetColorPayload {
    pub reserved1: u8,
    pub hue: u16,
    pub saturation: u16,
    pub brightness: u16,
    pub kelvin: u16,
    pub duration: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct StateServiceResponse {
    pub service: u8,
    pub port: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SetLightPowerPayload {
    pub level: u16,
    pub duration: u32,
}

impl SetLightPowerPayload {
    pub fn new(on: bool, duration: u32) -> SetLightPowerPayload {
        match on {
            true => SetLightPowerPayload { level: 65535, duration },
            false => SetLightPowerPayload { level: 0, duration },
        }
    }
}