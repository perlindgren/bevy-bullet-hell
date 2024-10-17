//! test of nonblock for raw mouse input
//! adjust path to desired mouse device

use nonblock;
use std::fs::File;

fn main() {
    let path = "/dev/input/mouse1";
    println!("waiting on input from: {:?}", path);
    let file = File::open(path).unwrap();
    let mut nb = nonblock::NonBlockingReader::from_fd(file).unwrap();

    let mut data = vec![];
    loop {
        match nb.read_available(&mut data) {
            Ok(n) => {
                if n == 3 {
                    println!("data {:?}", data);
                    break;
                }
            }
            _ => {}
        }
    }
}
