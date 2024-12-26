use std::net::TcpListener;
use std::thread::spawn;

use tungstenite::accept;


fn main() {
    let server = TcpListener::bind("127.0.0.1:1337").unwrap();

    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();

                if msg.is_binary() || msg.is_text() {
                    websocket.send(msg).unwrap();
                }
            }
        });
    }
}