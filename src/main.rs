/*
 * [Reservoir sampling](https://en.wikipedia.org/wiki/Reservoir_sampling)
 * [reservoir-sampling](https://github.com/DesmondWillowbrook/rs-reservoir-sampling)
 * [reservoir-rs](https://github.com/npryce/reservoir-rs)
 *
 * cargo test -- --nocapture
 */
extern crate rand;

use clap::Parser;
use rand::{
    Rng,
    thread_rng,
    distributions::{
        Uniform,
        Distribution,
    },
};
use std::env::args;
use std::io::{
    stdin,
    BufRead,
};
use std::iter::Iterator;



fn fill<I, T>(iter: &mut I, size: usize) -> Vec<T>
    where I: Iterator<Item=T>
{
    iter.take(size).collect::<Vec<_>>()
}



fn l<R, I, T>(iter: &mut I, size: usize, rng: &mut R) -> Vec<T>
    where
        R: Rng + ?Sized,
        I: Iterator<Item=T>
{
    let mut samples = fill(iter, size);
    if samples.len() < size {
        // There isn't enough items to sample.
        // This is the maximum number of items we can get.
        return samples;
    }

    let random_index = Uniform::new(0, size);

    let mut W: f64 = (rng.gen::<f64>().ln() / size as f64).exp();
    let mut i = size;

    loop {
        i += (rng.gen::<f64>().ln() / (1.0 - W).ln()).floor() as usize + 1;

        match iter.nth(i) {
            Some(n) => {
                samples[random_index.sample(rng)] = n;
                W *= (rng.gen::<f64>().ln() / size as f64).exp();
            }
            None => break,
        }
    };

    samples
}



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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_short() {
        let result = fill(&mut(0usize..5), 7);
        assert_eq!(result, (0usize..5).into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn fill_long() {
        let result = fill(&mut(0usize..100), 7);
        assert_eq!(result, (0usize..7).into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn l_short() {
        let mut rng = thread_rng();
        let result = l(&mut(0usize..5), 7, &mut rng);
        assert_eq!(result, (0usize..5).into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn l_long() {
        let mut rng = thread_rng();
        let result = l(&mut(-100isize..100), 7, &mut rng);
        assert!(result.iter().all(|&v| -100 <= v && v < 100));
    }
}
