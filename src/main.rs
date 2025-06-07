mod handshake;
mod frame;



use std::process::Command;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use frame::{read_frame, send_text, send_close};




#[tokio::main]
async fn main()
{
    //start a TCP listener on localhost:9001, where we wait for connections.
    let listener = TcpListener::bind("127.0.0.1:9001")
        .await.expect("Failed to bind ");
    
    println!("clearing console");
    Command::new("clear").status().expect("Failed to run 'clear'");
    println!("listening on ws://127.0.0.1:9001");

    loop{
        let (mut socket, addr) = listener.accept().await.expect("Failed to accept");
        println!("new connection from {:?}", addr);

        //spawn a new task to handle this connection
        tokio::spawn(
            async move {
                let mut buffer = [0u8;1024]; //read upto 1KB
                match socket.read(&mut buffer).await
                {
                    Ok(n) if n == 0 => return,
                    Ok(n) => {
                        let req = String::from_utf8_lossy(&buffer[..n]);
                        println!("Request: \n{}", req);

                        if let Some(response) = handshake::generate_handshake_response(&req)
                        {
                            if socket.write_all(response.as_bytes()).await.is_err()
                            {
                                return;
                            }
                            println!("Handshake completed with {}\n", addr);
                        }
                        else
                        {
                            println!("Invalid handshake from: {}", addr);
                            return;
                        }
                    },
                    Err(e) => {
                        println!("Failed to read from socket: {}", e);
                        return;
                    },
                }
                
                //websocket loop
                loop
                {
                    match read_frame(&mut socket).await
                    {
                        Some(frame) =>
                        {
                            match frame.opcode
                            {
                                0x1 => {
                                    if let Ok(text) = String::from_utf8(frame.payload)
                                    {
                                        println!("Received text: {}", text);
                                        //echo reply
                                        if let Err(e) = send_text(&mut  socket, &format!("Reply: {}", text)).await
                                        {
                                            println!("failed to reply: {}", e);
                                            return;
                                        }
                                    }
                                },
                                0x8 => {
                                    //close frame
                                    println!("Close frame received. Closing connection..");
                                    let _ = send_close(&mut socket).await;
                                    return;
                                },
                                other => {
                                    println!("unsupported Opcode: {}", other);
                                },
                            }
                        },

                        None => return,
                    }
                }
            }
        );
    }
}



