// for websocket frame parsing

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct Frame
{
    pub opcode: u8,
    pub payload: Vec<u8>,
    pub fin: bool,
}


pub async fn read_frame(socket: &mut TcpStream)
-> Option<Frame>
{
    let mut buf = [0u8; 1024];
    let n = socket.read(&mut buf).await.ok()?;

    if n == 0 
    {
        return None;
    }

    let fin = buf[0] & 0b1000_000 != 0;
    let opcode = buf[0] & 0x0F;
    let masked = buf[1] & 0b1000_0000 != 0;
    let mut payload_len = (buf[1] & 0b0111_1111) as usize;
    let mut index = 2;

    //handling extended payload lengths
    if payload_len == 126 
    {
        payload_len = ((buf[2] as usize) << 8 | buf[3] as usize) as usize;
        index += 2;
    }
    else if payload_len == 127 
    {
        println!("large payoads not supported in this demo");
        return None;
    }

    if !masked {
        return None;
    }
    let mask_key = &buf[index..index + 4];
    index +=4;
    if index + payload_len > n 
    {
        return None;
    }
    let masked_data = &buf[index..index + payload_len];
    let mut data = Vec::with_capacity(payload_len);

    for (i, byte) in masked_data.iter().enumerate() 
    {
        data.push(byte^mask_key[i % 4]);
    }

    Some(Frame{
        opcode,
        payload: data,
        fin,
    })
}


//helper function: send a text frame back
pub async fn send_text(socket: &mut TcpStream, message: &str)
-> tokio::io::Result<()>
{
    let bytes = message.as_bytes();
    let len = bytes.len();
    let mut frame = Vec::new();

    frame.push(0x81); //FIN + text frame
    if len <= 125
    {
        frame.push(len as u8);
    }
    else if  len <= 65535 
    {
        frame.push(126);
        frame.push((len>>8) as u8);
        frame.push((len & 0xFF) as u8);
    }
    else
    {
        panic!("message too large"); //for simplicity
    }

    frame.extend_from_slice(bytes);
    socket.write_all(&frame).await
}

pub async fn send_close (socket: &mut TcpStream)
-> tokio::io::Result<()>
{
    let frame = [0x88, 0x00];   //FIN + Close, no payload

    socket.write_all(&frame).await
}