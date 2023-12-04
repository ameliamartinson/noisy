use std::io::{self, Read, Write};
use rand::distributions::{Bernoulli, Distribution};

fn main() {
    let mut buffer = Vec::<u8>::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _ = handle.read_to_end(&mut buffer);

    let output = flip_bits(buffer, 0.01);

    let _ = io::stdout().write_all(&output);
}

fn flip_bits(data: Vec<u8>, probability: f64) -> Vec<u8> {
    let mut data = data.clone();
    for bit in &mut data {
        let dist = Bernoulli::new(probability).unwrap();
        let mut err_byte = 0_u8;
        for i in 0..8 {
            let flips = dist.sample(&mut rand::thread_rng());
            if flips {
                err_byte += 1<<i;
            }
        }
        *bit ^= err_byte;
    } 
    data
}
