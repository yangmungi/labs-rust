extern crate rand;

use std::io;
use std::env::args;
use std::i32;

use rand::Rng;

fn main() {
    let args = &mut args();
    let input_number = &args.nth(1)
        .expect("must have exactly 1 arg");

    let number = match i32::from_str_radix(input_number, 10) {
        Ok(output) => output,
        Err(e) => panic!(e)
    };

    let thread_rng = &mut rand::thread_rng();

    for _ in 0..number {
        let rn = thread_rng.gen_range(1, 10);
        print!("{}", rn);
    }
}
