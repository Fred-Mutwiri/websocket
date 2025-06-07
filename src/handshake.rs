// for the HTTP Upgrade logic

use sha1::Sha1;
use digest::Digest;
use base64::engine::general_purpose;
use base64::Engine;

pub fn generate_handshake_response(req: &str)
->Option<String>
{
    let key_line = req.lines().find(|line| line.to_lowercase().starts_with("sec-websocket-key"))?;
    let key = key_line.split(':').nth(1).unwrap().trim();
    let magic_guid = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
    let combined = format!("{}{}", key, magic_guid);
    let mut hasher = Sha1::new();
    hasher.update(combined.as_bytes());
    let hashed = hasher.finalize();

    //base64 encode the result
    let encoded = general_purpose::STANDARD.encode(hashed);

    Some(format!(
        "HTTP/1.1 101 Switching Protocols\r\n\
         Upgrade: websocket\r\n\
         Connection: Upgrade\r\n\
         Sec-WebSocket-Accept: {}\r\n\r\n", encoded
    ))

}