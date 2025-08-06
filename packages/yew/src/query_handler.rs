use std::net::UdpSocket;
use std::io::Read;

/// Handler for processing query operations
/// Receives query operation data via UDP socket and processes it through query operations
pub fn process_query_stream() -> Result<String, String> {
    let socket = match UdpSocket::bind("127.0.0.1:8082") {
        Ok(socket) => socket,
        Err(_) => return Err("Failed to bind UDP socket".to_string())
    };
    
    let mut buffer = [0u8; 1024];
    
    //SOURCE
    let read_result = match socket.recv_from(&mut buffer) {
        Ok((bytes, _addr)) => bytes,
        Err(_) => return Err("Failed to receive query data from UDP socket".to_string())
    };
    
    if read_result > 0 {
        let query_data = String::from_utf8_lossy(&buffer[..read_result]).to_string();
        match crate::query_engine::handle_query_operations(query_data) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("Query engine error: {}", e))
        }
    } else {
        Err("No query data received".to_string())
    }
} 