use bincode::{deserialize, serialize};
use std::net::UdpSocket;
use udp::*;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        println!("{:?}", "Waiting for input");
        let mut buf = [0; 256];
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("from {}, {:?}, amt {}", src, buf, amt);

        let mut data: Data = deserialize(&buf).expect("deserialize");

        println!("data {:?}", data);
        data.is *= 10.;

        let buf = serialize(&data).expect("seriazile");

        let amt = socket.send_to(&buf, src).expect("failed send");
        println!("sent amt {:?}", amt);
    } // the socket is closed here
    Ok(())
}
