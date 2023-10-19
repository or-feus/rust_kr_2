extern crate greprs;

use std::env;
use std::process;
use greprs::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    //
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });
    //
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);
    //
    // if let Err(e) = greprs::run(config) {
    //     println!("Application error: {}", e);
    //     process::exit(1);
    // }
    //
    //
    //


    let buffer: &mut [i32];
    let coefficients: [i64; 12];
    let qlp_shift: i16;

    // for i in 12..buffer.len() {
    //     let prediction = coefficients.iter().zip(&buffer[i - 12..i]).map(|(&c, &s)| c * s as i64).sum::<i64>() >> qlp_shift;
    //
    //     let delta = buffer[i];
    //     buffer[i] = prediction as i32 + delta;
    // }
}
