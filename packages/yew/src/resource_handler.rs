use std::net::TcpStream;
use std::io::Read;

/// Handler for processing external resource operations
/// Receives external resource data via TCP stream and processes it through resource operations
pub fn process_external_resource_stream() -> Result<String, String> {
    let mut stream = match TcpStream::connect("127.0.0.1:8083") {
        Ok(stream) => stream,
        Err(_) => return Err("Failed to connect to TCP stream".to_string()),
    };

    let mut buffer = [0u8; 1024];

    //SOURCE
    let read_result = match stream.read(&mut buffer) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Failed to read from TCP stream".to_string()),
    };

    if read_result > 0 {
        let resource_data = String::from_utf8_lossy(&buffer[..read_result]).to_string();
        match crate::resource_engine::handle_external_resource_operations(resource_data) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("Resource engine error: {}", e)),
        }
    } else {
        Err("No external resource data received".to_string())
    }
} 