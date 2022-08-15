use log::debug;
use serde::{Serialize, Deserialize};

/* Serialize from self to binary */
pub trait BinarySerializable {
    fn serialize(&self) -> Vec<u8>;
}

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
    pub fn new(sequence: u8, packet_type: u16, mac_address: Option<[u8; 6]>) -> Header {
        let mut target: [u8; 8] = [0; 8];

        if let Some(mac_address) = mac_address {
            debug!("Targeting specific mac address: {:?}", &mac_address);

            for i in 0..mac_address.len() {
                target[i] = mac_address[i];
            }
        }

        let reserved1: [u8; 6] = [0; 6];

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

impl BinarySerializable for SetLightPowerPayload {
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}
#[allow(dead_code)]
pub enum LifxPacket {
    /*
        Query packets
    */

    /* https://lan.developer.lifx.com/docs/querying-the-device-for-data#discovery */
    GetService = 2,

    /* https://lan.developer.lifx.com/docs/querying-the-device-for-data#device */
    GetHostFirware = 14,
    GetWifiInfo = 16,
    GetWifiFirmware = 18,
    GetPower = 20,
    GetLabel = 23,
    GetVersion = 32,
    GetInfo = 34,
    GetLocation = 48,
    GetGroup = 51,
    EchoRequest = 58,

    /* https://lan.developer.lifx.com/docs/querying-the-device-for-data#light */
    GetColor = 101,
    GetLightPower = 116,
    GetInfrared = 120,
    GetHevCycle = 142,
    GetHevCycleConfiguration = 145,
    GetLastHevCycleResult = 148,

    /* https://lan.developer.lifx.com/docs/querying-the-device-for-data#multizone */
    GetColorZones = 502,
    GetMultiZoneEffect = 507,
    GetExtendedColorZones = 511,

    /* https://lan.developer.lifx.com/docs/querying-the-device-for-data#relay */
    GetRPower = 816,

    /* https://lan.developer.lifx.com/docs/querying-the-device-for-data#tile */
    GetDeviceChain = 701,
    Get64 = 707,
    GetTileEffect = 718,

    /* 
        Change packets
    */

    /* https://lan.developer.lifx.com/docs/changing-a-device#device */
    SetPower = 21,
    SetLabel = 24,
    SetReboot = 38,
    SetLocation = 49,
    SetGroup = 52,

    /* https://lan.developer.lifx.com/docs/changing-a-device#light */
    SetColor = 102,
    SetWaveform = 103,
    SetLightPower = 117,
    SetWaveformOptional = 119,
    SetInfrared = 122,
    SetHevCycle = 143,
    SetHevCycleConfiguration = 146,

    /* https://lan.developer.lifx.com/docs/changing-a-device#multizone */
    SetColorZones = 501,
    SetMultiZoneEffect = 508,
    SetExtendedColorZones = 510,

    /* https://lan.developer.lifx.com/docs/changing-a-device#relay */
    SetRPower = 817,

    /* https://lan.developer.lifx.com/docs/changing-a-device#tile */
    SetUserPosition = 703,
    Set64 = 715,
    SetTileEffect = 719,
}