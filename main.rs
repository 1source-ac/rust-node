use tungstenite::{connect, Message};
use std::collections::HashMap;
use serde_json::{Value, Error};
use std::process::Command;

fn main() {
    // Define the WebSocket server URL
    let url = "ws://your-websocket-server.com";

    // Establish a connection
    let (mut socket, _) = connect(url.parse().unwrap()).expect("Failed to connect");

    // Define a mapping of commands to executable paths
    let mut commands_map = HashMap::new();
    commands_map.insert("run ableton", "ableton.exe");

    // Listen for messages
    loop {
        let msg = socket.read_message().expect("Error reading message");

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
        if command_type == "exec" {
            if let Some(action) = v["action"].as_str() {
                if let Some(exec_path) = commands_map.get(action) {
                    Command::new(exec_path).spawn().expect("Failed to execute command");
                } else {
                    eprintln!("Unknown command: {}", action);
                }
            }
        }
    }
    Ok(())
}
