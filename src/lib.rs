/*
 * [Reservoir sampling](https://en.wikipedia.org/wiki/Reservoir_sampling)
 * [reservoir-sampling](https://github.com/DesmondWillowbrook/rs-reservoir-sampling)
 * [reservoir-rs](https://github.com/npryce/reservoir-rs)
 * [The Rust Rand Book](https://rust-random.github.io/book/intro.html)
 *
 * cargo test -- --nocapture
 */
extern crate rand;
use crate::rand::distributions::Distribution;

use rand::{distributions::Uniform, Rng};
use std::collections::BinaryHeap;
use std::iter::Iterator;

fn fill<I, T>(iter: &mut I, size: usize) -> Vec<T>
where
    I: Iterator<Item = T>,
{
    iter.take(size).collect::<Vec<_>>()
}

pub fn l<R, I, T>(iter: &mut I, size: usize, rng: &mut R) -> Vec<T>
where
    R: Rng + ?Sized,
    I: Iterator<Item = T>,
{
    let mut samples = fill(iter, size);
    if samples.len() < size {
        // There isn't enough items to sample.
        // This is the maximum number of items we can get.
        eprintln!("WARN: Population smaller than sample size");
        return samples;
    }

    let random_index = Uniform::new(0, size);

    let mut w: f64 = (rng.gen::<f64>().ln() / size as f64).exp();

    loop {
        // NOTE: 'steps_until_next_candidate` formerly known as `i`.
        // NOTE: Difference with the original algorithm.  Since we are using `iter.nth()`, `i`
        // represents the distance until the next candidate and NOT it position in the stream.
        //i = (rng.gen::<f64>().ln() / (1.0 - w).ln()).floor() as usize + 1;
        // NOTE: Since `iter.nth(0)` returns the next item, we've dropped the `+ 1`.
        let steps_until_next_candidate = (rng.gen::<f64>().ln() / (1.0 - w).ln()).floor() as usize;

        match iter.nth(steps_until_next_candidate) {
            Some(n) => {
                samples[random_index.sample(rng)] = n;
                w *= (rng.gen::<f64>().ln() / size as f64).exp();
            }
            None => break,
        }
    }

    samples
}

//#[derive(Eq, Ord, PartialOrd, PartialEq)]
//#[derive(PartialOrd, PartialEq)]
#[derive(Debug)]
pub struct WeightedItem<T> {
    weight: f64,
    item: T,
}

// Converting the default max-heap to a min-heap.
// https://doc.rust-lang.org/stable/src/alloc/collections/binary_heap/mod.rs.html#30
impl<T> Ord for WeightedItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Notice that the we flip the ordering on costs.
        other.weight.partial_cmp(&self.weight).unwrap()
    }
}

impl<T> PartialOrd for WeightedItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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

pub fn a_exp_j<R, I, T>(stream: &mut I, size: usize, rng: &mut R) -> Vec<T>
where
    R: Rng + ?Sized,
    I: Iterator<Item = (f64, T)>,
    T: std::cmp::Ord + std::fmt::Debug,
{
    if size == 0 {
        return vec![];
    }

    // NOTE that BinaryHeap is a maxHeap so we change the polarity of the weights/keys to make it a
    // minHeap.
    let mut heap = stream
        .into_iter()
        .take(size)
        .map(|(w, item)| WeightedItem {
            weight: rng.gen::<f64>().powf(1. / w),
            item: item,
        })
        .collect::<BinaryHeap<WeightedItem<T>>>();

    if let Some(min) = heap.peek() {
        //println!("Weighted sampling...");
        //println!("HEAP: {:?}", heap);
        //println!("H.min: {:?}", min.weight);
        let mut x = rng.gen::<f64>().ln() / (min.weight).ln();
        for (weight, item) in stream {
            //println!("x {:?} w: {:?} i: {:?}", x, weight, item);
            x -= weight;
            if x <= 0. {
                //eprintln!("{}", x);
                let t = (heap.pop().unwrap().weight).powf(weight);
                let r = rng.gen_range(t..1.).powf(1. / weight);
                heap.push(WeightedItem {
                    weight: r,
                    item: item,
                });

                x = rng.gen::<f64>().ln() / (heap.peek().unwrap().weight).ln();
            }
        }
    }
    //println!("HEAP: {:?}", heap);

    heap.into_iter().map(|x| x.item).collect()
}

#[cfg(test)]
mod tests_l {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn fill_short() {
        let result = fill(&mut (0usize..5), 7);
        assert_eq!(result, (0usize..5).into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn fill_long() {
        let result = fill(&mut (0usize..100), 7);
        assert_eq!(result, (0usize..7).into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn not_enough_population() {
        let mut rng = thread_rng();
        let result = l(&mut (0usize..5), 7, &mut rng);
        assert_eq!(result, (0usize..5).into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn sufficient_population_size() {
        let mut rng = thread_rng();
        let result = l(&mut (-100isize..100), 7, &mut rng);
        assert!(result.iter().all(|&v| -100 <= v && v < 100));
    }
}

#[cfg(test)]
mod tests_l_chi_squarred {
    use super::*;
    use rand::thread_rng;
    use statrs::function::gamma::gamma_li;

    fn chi_distance(dataset: &[i32]) -> f64 {
        let expected = f64::from(dataset.iter().sum::<i32>()) / dataset.len() as f64;
        dataset
            .iter()
            .fold(0., |acc, &elt| acc + (elt as f64 - expected).powf(2.))
            / expected
    }

    fn chi2_probability(dof: f64, distance: f64) -> f64 {
        1. - gamma_li(dof * 0.5, distance * 0.5)
    }

    fn chi2_uniform(dataset: &[i32], significance: f64) -> bool {
        let d = chi_distance(&dataset);
        chi2_probability(dataset.len() as f64 - 1., d) > significance
    }

    #[test]
    fn test_chi() {
        // https://rosettacode.org/wiki/Verify_distribution_uniformity/Chi-squared_test#Rust
        let mut rng = thread_rng();
        let sample = l(&mut (0i32..1000000), 7, &mut rng);
        assert!(sample.iter().all(|&v| 0 <= v && v < 1000000));

        let dsets = vec![
            vec![199809, 200665, 199607, 200270, 199649],
            vec![522573, 244456, 139979, 71531, 21461],
            vec![5735, 8609, 5317, 2975, 3307, 7487, 3451],
        ];

        for ds in dsets {
            println!("Data set: {:?}", ds);
            let d = chi_distance(&ds);
            print!("Distance: {:.6} ", d);
            print!(
                "Chi2 probability: {:.6} ",
                chi2_probability(ds.len() as f64 - 1., d)
            );
            print!("Uniform? {}\n", chi2_uniform(&ds, 0.05));
        }
        println!("Data set: {:?}", sample.as_slice());
        let d = chi_distance(&sample.as_slice());
        print!("Distance: {:.6} ", d);
        print!(
            "Chi2 probability: {:.6} ",
            chi2_probability(sample.len() as f64 - 1., d)
        );
        print!("Uniform? {}\n", chi2_uniform(&sample.as_slice(), 0.05));
        assert!(chi2_uniform(sample.as_slice(), 0.05));
    }
}

#[cfg(test)]
mod tests_a_exp_j {
    use super::*;
    use rand::{
        distributions::{Alphanumeric, DistIter, Standard},
        rngs::ThreadRng,
        thread_rng,
    };
    use std::iter::{Take, Zip};

    fn generate_random<W, V>(
        w_size: usize,
        i_size: usize,
    ) -> Zip<<W as IntoIterator>::IntoIter, <V as IntoIterator>::IntoIter>
    where
        W: IntoIterator<IntoIter = Take<DistIter<Standard, ThreadRng, f64>>>,
        V: IntoIterator<IntoIter = Take<DistIter<Alphanumeric, ThreadRng, u8>>>,
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
        let samples = a_exp_j(&mut weights.zip(items), 4, &mut thread_rng());
        assert!(samples.len() == 0);
    }

    #[test]
    fn empty() {
        // There is nothing in the stream.
        let samples = a_exp_j(
            &mut vec![0.5; 0].into_iter().map(|x| x as f64).zip("".chars()),
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
