use std::net::TcpStream;
use std::io::Read;

/// Handler for processing file operations
/// Receives file operation data via TCP stream and processes it through file operations
pub fn process_file_stream() -> Result<String, String> {
    let mut stream = match TcpStream::connect("127.0.0.1:8080") {
        Ok(stream) => stream,
        Err(_) => return Err("Failed to connect to TCP stream".to_string())
    };
    
    let mut buffer = [0u8; 1024];
    
    //SOURCE
    let read_result = match stream.read(&mut buffer) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Failed to read from TCP stream".to_string())
    };
    
    if read_result > 0 {
        let file_data = String::from_utf8_lossy(&buffer[..read_result]).to_string();
        match crate::file_engine::handle_file_operations(file_data) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("File engine error: {}", e))
        }
    } else {
        Err("No file data received".to_string())
    }
} 