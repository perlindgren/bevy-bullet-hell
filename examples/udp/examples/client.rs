use std::net::UdpSocket;

use bincode::{deserialize, serialize};
use udp::*;

fn main() {
    let data = Data {
        this: 0.25,
        is: 0.8,
        a: 7,
        _test: -7,
    };

    let mut buf = serialize(&data).expect("serialize error");

    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    socket
        .send_to(&buf, "127.0.0.1:34254")
        .expect("couldn't send data");
    let (amt, _src) = socket.recv_from(&mut buf).expect("failed receive");
    println!("received amt {:?}, {:?}", amt, &buf[0..amt]);

    let new_data: Data = deserialize(&buf).expect("desiriaze error");
    println!("data {:?}", new_data);
}
