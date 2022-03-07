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



#[cfg(test)]
mod tests_a_exp_j {
    use super::*;
    use rand::distributions::{Alphanumeric, Uniform, Standard};
    use std::iter::Zip;
    use rand::rngs::ThreadRng;
    use rand::distributions::DistIter;
    use std::iter::Take;

    fn generate_random<W, V>(w_size: usize, i_size: usize)
        -> Zip<<W as IntoIterator>::IntoIter, <V as IntoIterator>::IntoIter>
        where
            //W: IntoIterator,
            W: IntoIterator<IntoIter = std::iter::Take<DistIter<Standard, ThreadRng, f64>>>,
            //V: IntoIterator,
            V: IntoIterator<IntoIter = std::iter::Take<DistIter<Alphanumeric, ThreadRng, u8>>>,
    {
        let weights_rng = thread_rng();
        let weights = (weights_rng).sample_iter(Standard).take(w_size);
        let items_rng = thread_rng();
        let items = (items_rng).sample_iter(Alphanumeric).take(i_size);
        weights.zip(items)
    }

    #[test]
    fn sfasdfa() {
        // There is nothing in the stream.
        let weights_rng = thread_rng();
        let weights = (weights_rng).sample_iter(Standard).take(0);
        let items_rng = thread_rng();
        let items = (items_rng).sample_iter(Alphanumeric).take(0);
        let samples = a_exp_j(
            &mut weights.zip(items),
            4,
            &mut thread_rng(),
            );
        assert!(samples.len() == 0);
    }

    #[test]
    fn empty() {
        // There is nothing in the stream.
        let samples = a_exp_j(
            &mut vec![0.5; 0]
                .into_iter()
                .map(|x| x as f64)
                .zip("".chars()),
            4,
            &mut thread_rng(),
            );
        assert!(samples.len() == 0);
    }

    #[test]
    fn none() {
        // Asking for ZERO sample.
        let samples = a_exp_j(
            &mut vec![0.5; 10]
                .into_iter()
                .map(|x| x as f64)
                .zip((0..10).into_iter()),
            0,
            &mut thread_rng(),
            );
        assert!(samples.len() == 0);
    }

    #[test]
    fn not_enough_population() {
        // Asking for more samples than there are items in the stream.
        let samples = a_exp_j(
            &mut vec![0.5; 5]
                .into_iter()
                .map(|x| x as f64)
                .zip((0..5).into_iter()),
            7,
            &mut thread_rng(),
            );
        assert!(samples.len() == 5);
    }


    #[test]
    fn sufficient_population_size() {
        // A normal use case scenario where m < n.
        let samples = a_exp_j(
            &mut vec![0.5; 10]
                .into_iter()
                .map(|x| x as f64)
                .zip((0..10).into_iter()),
            7,
            &mut thread_rng(),
            );
        assert!(samples.len() == 7);
    }


    #[test]
    fn samuel_larkin() {
        let samples = a_exp_j(
            &mut vec![0.5; 12]
                .into_iter()
                .map(|x| x as f64)
                .zip("SamuelLarkin".chars()),
            4,
            &mut thread_rng(),
            );
        println!("SAMPLES: {:?}", samples);
        assert!(samples.len() == 4);
    }
}
