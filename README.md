a miniature websocket server with the ability to take a frame and parse it from its binary form and echo the payload back as a reply.
it's modularized to contain each logic in it's own file and handle a persistent connection with the client.
it's not a replacement of tokio::tungstenite but just a low-level server for learning purposes, to understand the inne working of a web-socket server.

it's implemented in rust.
