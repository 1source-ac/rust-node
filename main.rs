use tungstenite::connect;
use serde_json::{Value, Error};
use std::collections::HashMap;
use wol::send_wol;
use url::Url;
use std::env;

fn main() {
    // Define the WebSocket server URL
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <websocket_server_url>", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];

    // Establish a connection
    let parsed_url = Url::parse(url).unwrap();
    let (mut socket, _) = connect(parsed_url).expect("Failed to connect");

    // Define a mapping of commands to executable paths
    let mut commands_map = HashMap::new();
    commands_map.insert("run ableton", "C:/ProgramData/Ableton/Live 11 Suite/Program/Ableton Live 11 Suite.exe");

    // Listen for messages
    loop {
        let msg = socket.read().expect("Error reading message");

        if msg.is_text() || msg.is_binary() {
            if let Err(e) = handle_command(msg.to_text().unwrap(), &commands_map) {
                eprintln!("Failed to handle command: {}", e);
            }
        }
    }
}

fn handle_command(command_str: &str, commands_map: &HashMap<&str, &str>) -> Result<(), Error> {
    let v: Value = serde_json::from_str(command_str)?;
    if let Some(command_type) = v["type"].as_str() {
        match command_type {
            "exec" => {
                if let Some(action) = v["action"].as_str() {
                    if let Some(exec_path) = commands_map.get(action) {
                        std::process::Command::new(exec_path).spawn().expect("Failed to execute command");
                    } else {
                        eprintln!("Unknown command: {}", action);
                    }
                }
            },
            "wol" => {
                if let Some(mac_address) = v["mac"].as_str() {
                    // Assuming your MAC address string is in the format "AA:BB:CC:DD:EE:FF"
                    let bytes: Vec<u8> = mac_address
                    .split(':')
                    .map(|s| u8::from_str_radix(s, 16).expect("Failed to parse byte"))
                    .collect();

                    if bytes.len() != 6 {
                        panic!("Invalid MAC address");
                    }

                    let wol_mac = wol::MacAddr([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]]);

                    send_wol(wol_mac, None, None).unwrap_or_else(|err| {
                        eprintln!("Failed to send Wake-on-LAN: {}", err);
                    });
                } else {
                    eprintln!("MAC address not provided for Wake-on-LAN");
                }
            },
            _ => eprintln!("Unknown command type: {}", command_type),
        }
    }
    Ok(())
}

