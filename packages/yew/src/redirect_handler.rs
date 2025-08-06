use socket2::{Socket, Domain, Type};
use std::net::SocketAddr;

/// Handler for processing redirect operations
/// Receives redirect operation data via socket and processes it through redirect operations
pub fn process_redirect_stream() -> Result<String, String> {
    let socket = match Socket::new(Domain::IPV4, Type::DGRAM, None) {
        Ok(socket) => socket,
        Err(_) => return Err("Failed to create socket".to_string())
    };
    
    let addr: SocketAddr = "127.0.0.1:8083".parse().unwrap();
    if let Err(_) = socket.bind(&addr.into()) {
        return Err("Failed to bind socket".to_string());
    }
    
    let mut buffer = [std::mem::MaybeUninit::<u8>::uninit(); 1024];
    
    //SOURCE
    let read_result = match socket.recv_from(&mut buffer) {
        Ok((bytes, _addr)) => bytes,
        Err(_) => return Err("Failed to receive redirect data from socket".to_string())
    };
    
    if read_result > 0 {
        let redirect_data = unsafe {
            String::from_utf8_lossy(
                std::mem::transmute::<&[std::mem::MaybeUninit<u8>], &[u8]>(&buffer[..read_result])
            ).to_string()
        };
        match crate::redirect_engine::handle_redirect_operations(redirect_data) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("Redirect engine error: {}", e))
        }
    } else {
        Err("No redirect data received".to_string())
    }
} 