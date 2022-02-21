/*
 * [Reservoir sampling](https://en.wikipedia.org/wiki/Reservoir_sampling)
 * [reservoir-sampling](https://github.com/DesmondWillowbrook/rs-reservoir-sampling)
 * [reservoir-rs](https://github.com/npryce/reservoir-rs)
 */
extern crate rand;

use rand::{
    Rng,
    thread_rng,
    distributions::{
        Uniform,
        Distribution,
    },
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
    if (samples.len() < size) {
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



fn main() {
    let mut rng = thread_rng();
    println!("Hello, world!");
    println!("{:?}", fill(&mut(0usize..100), 7));
    println!("{:?}", l(&mut(0usize..100), 7, &mut rng));
}
