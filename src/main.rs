use rand::distributions::{Bernoulli, Distribution};
use std::io::{self, Read, Write};

const MASK: [u8; 8] = [0x80, 0x40, 0x20, 0x10, 0x8, 0x4, 0x2, 0x1];

fn main() {
    let mut buffer = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _ = handle.read_to_end(&mut buffer);

    let mut output = Vec::with_capacity(buffer.len());
    let dist = Bernoulli::new(0.01).unwrap();

    for byte in buffer {
        let mut err_byte = 0;
        for byte in MASK {
            if dist.sample(&mut rand::thread_rng()) {
                err_byte |= byte;
            }
        }
        output.push(byte ^ err_byte);
    }

    let _ = io::stdout().write_all(&output);
}
