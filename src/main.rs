// src/main.rs
use dronesim::TelemetryPacket;
use rand::Rng;
use serde_json;
use std::{io::Write, net::TcpStream, thread, time::Duration};
use chrono::Utc;

fn path_positions(
    waypoints: &[(f64, f64)],
    steps_per_segment: usize,
) -> impl Iterator<Item = (f64, f64)> + '_ {
    waypoints
        .windows(2)
        .flat_map(move |seg| {
            let (lat0, lon0) = seg[0];
            let (lat1, lon1) = seg[1];
            (0..steps_per_segment).map(move |i| {
                let t = i as f64 / steps_per_segment as f64;
                let lat = lat0 + (lat1 - lat0) * t;
                let lon = lon0 + (lon1 - lon0) * t;
                (lat, lon)
            })
        })
        .chain(std::iter::once(waypoints[0]))
}

fn simulate_flight() -> std::io::Result<()> {
    let waypoints = vec![
        (25.276987, 55.296249),
        (25.277500, 55.296249),
        (25.277500, 55.297000),
        (25.276987, 55.297000),
    ];
    let mut path_iter = path_positions(&waypoints, 50);

    let mut rng     = rand::thread_rng();
    let mut battery = 100.0_f32;
    let mut pitch   =  0.0_f32;
    let mut roll    =  0.0_f32;
    let mut yaw     =  0.0_f32;

    let mut stream = TcpStream::connect("127.0.0.1:9000")?;

    loop {
        let (lat, lon) = path_iter.next().unwrap_or_else(|| {
            let mut new_iter = path_positions(&waypoints, 50);
            let pt = new_iter.next().unwrap();
            path_iter = new_iter;
            pt
        });

        battery = (battery - 0.05).max(0.0);
        pitch  += rng.gen_range(-1.0..1.0);
        roll   += rng.gen_range(-1.0..1.0);
        yaw     = (yaw + rng.gen_range(-5.0..5.0)) % 360.0;

        let packet = TelemetryPacket {
            drone_id:    1,
            timestamp:   Utc::now().timestamp_millis() as u64,
            latitude:    lat,
            longitude:   lon,
            altitude:    rng.gen_range(0.0_f32..120.0_f32),
            pitch,
            roll,
            yaw,
            battery,
            throttle:    rng.gen_range(0.0_f32..1.0_f32),
            speed:       rng.gen_range(0.0_f32..15.0_f32),
            flight_mode: "Auto".into(),
            gps_fix:     true,
        };

        let serialized = serde_json::to_string(&packet).unwrap();
        stream.write_all(serialized.as_bytes())?;
        stream.write_all(b"\n")?;

        thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    simulate_flight().unwrap();
}

