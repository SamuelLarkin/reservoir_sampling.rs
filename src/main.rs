/*
 * [Reservoir sampling](https://en.wikipedia.org/wiki/Reservoir_sampling)
 * [reservoir-sampling](https://github.com/DesmondWillowbrook/rs-reservoir-sampling)
 * [reservoir-rs](https://github.com/npryce/reservoir-rs)
 *
 * cargo test -- --nocapture
 */

use clap::Parser;
use rand::{
    Rng,
    thread_rng,
    distributions::{
        Uniform,
        Distribution,
    },
};
use std::io::{
    stdin,
    BufRead,
};
use reservoir_sampling::l;



#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 10)]
    size: usize,
}



fn main() {
    let args = Args::parse();
    let input = stdin();
    let mut input_lines = input.lock().lines().map(|r| r.unwrap());
    let mut rng = thread_rng();
    let samples = l(
        &mut input_lines,
        args.size,
        &mut rng);
    for sample in samples {
        println!("{}", sample);
    }
}
