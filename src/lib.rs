use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct TelemetryPacket {
    pub drone_id: u8,
    pub timestamp: u64,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
    pub battery: f32,
    pub throttle: f32,
    pub speed: f32,
    pub flight_mode: String,
    pub gps_fix: bool,
    
}

