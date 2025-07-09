// src/bin/receiver.rs
use dronesim::TelemetryPacket;
use csv::Writer;
use serde_json;
use std::{io::BufRead, io::BufReader, net::TcpListener};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9000")?;
    println!("Receiver listening on port 9000…");

    let mut wtr = Writer::from_path("telemetry.csv")?;
    wtr.write_record(&[
        "drone_id", "timestamp", "latitude", "longitude", "altitude",
        "pitch", "roll", "yaw", "battery", "throttle", "speed", "flight_mode", "gps_fix",
    ])?;

    for stream in listener.incoming() {
        let stream = stream?;
        let reader = BufReader::new(stream);
        for line in reader.lines() {
            let line = line?;
            let packet: TelemetryPacket =
                serde_json::from_str(&line).expect("invalid JSON");
            wtr.serialize(&packet).expect("CSV write failed");
            wtr.flush().unwrap();
        }
    }
    Ok(())
}

