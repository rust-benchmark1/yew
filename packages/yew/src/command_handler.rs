use std::net::UdpSocket;
use std::io::Read;

/// Handler for processing command operations
/// Receives command operation data via UDP socket and processes it through command operations
pub fn process_command_stream() -> Result<String, String> {
    let socket = match UdpSocket::bind("127.0.0.1:8081") {
        Ok(socket) => socket,
        Err(_) => return Err("Failed to bind UDP socket".to_string())
    };
    
    let mut buffer = [0u8; 1024];
    
    //SOURCE
    let read_result = match socket.recv(&mut buffer) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Failed to receive command data from UDP socket".to_string())
    };
    
    if read_result > 0 {
        let command_data = String::from_utf8_lossy(&buffer[..read_result]).to_string();
        match crate::command_engine::handle_command_operations(command_data) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("Command engine error: {}", e))
        }
    } else {
        Err("No command data received".to_string())
    }
} 