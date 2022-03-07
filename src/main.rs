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
use std::collections::BinaryHeap;
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



//#[derive(Eq, Ord, PartialOrd, PartialEq)]
//#[derive(PartialOrd, PartialEq)]
#[derive(Debug)]
pub struct WeightedItem <T>
{
    weight: f64,
    item: T,
}


impl<T> Ord for WeightedItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.partial_cmp(&other.weight).unwrap()
    }
}


impl<T> PartialOrd for WeightedItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        //Some(self.weight.partial_cmp(&other.weight))
        self.weight.partial_cmp(&other.weight)
    }
}



impl<T> PartialEq for WeightedItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight.eq(&other.weight)
    }
}


impl<T> Eq for WeightedItem<T> {
    fn assert_receiver_is_total_eq(&self) {}
}



fn a_exp_j<R, I, T>(stream: &mut I, size: usize, rng: &mut R) -> Vec<T>
    where
        R: Rng + ?Sized,
        I: Iterator<Item=(f64, T)>,
        T: std::cmp::Ord + std::fmt::Debug
{
    if size == 0 {
        return vec![];
    }

    // NOTE that BinaryHead is a maxHeap so we change the polarity of the weights/keys to make it a
    // minHeap.
    let mut heap = stream
        .into_iter()
        .take(size)
        .map(|(w, item)| WeightedItem {
            weight: -rng.gen::<f64>().powf(1. / w),
            item: item,
        })
        .collect::<BinaryHeap<WeightedItem<T>>>();

    if let Some(min) = heap.peek() {
        //println!("Weighted sampling...");
        //println!("HEAP: {:?}", heap);
        //println!("H.min: {:?}", min.weight);
        let mut X = rng.gen::<f64>().ln() / -min.weight;
        for (weight, item) in stream {
            //println!("X {:?} w: {:?} i: {:?}", X, weight, item);
            X += weight;
            if X <= 0. {
                let t = (-heap.pop().unwrap().weight).powf(weight);
                let r = rng.gen_range(t..1.).powf(1. / weight);
                heap.push(WeightedItem {weight: -r, item: item});

                X = rng.gen::<f64>().ln() / -heap.peek().unwrap().weight;
            }
        }
    }
    //println!("HEAP: {:?}", heap);

    heap.into_iter().map(|x| x.item).collect()
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
mod tests_l {
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
    fn not_enough_population() {
        let mut rng = thread_rng();
        let result = l(&mut(0usize..5), 7, &mut rng);
        assert_eq!(result, (0usize..5).into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn sufficient_population_size() {
        let mut rng = thread_rng();
        let result = l(&mut(-100isize..100), 7, &mut rng);
        assert!(result.iter().all(|&v| -100 <= v && v < 100));
    }
}
