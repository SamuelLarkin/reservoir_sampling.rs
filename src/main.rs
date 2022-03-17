/*
 * [Reservoir sampling](https://en.wikipedia.org/wiki/Reservoir_sampling)
 * [reservoir-sampling](https://github.com/DesmondWillowbrook/rs-reservoir-sampling)
 * [reservoir-rs](https://github.com/npryce/reservoir-rs)
 *
 * cargo test -- --nocapture
 */

use clap::{
    Parser,
    Subcommand,
};
use rand::{
    SeedableRng,
    rngs::SmallRng,
    thread_rng,
};
use std::fs::File;
use std::io::{
    BufRead,
    BufReader,
    self,
};
use std::path::Path;
use reservoir_sampling::{
    a_exp_j,
    l,
};



#[derive(Parser)]
#[clap(name = "reservoir_sampling")]
#[clap(author, version, about="Sampling a stream", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}



#[derive(Subcommand)]
enum Commands {
    #[clap(arg_required_else_help=false)]
    /// Unweighted resevoir sampling
    Unweighted {
        /// Seed for reproducibility
        #[clap(long, parse(try_from_str))]
        seed: Option<u64>,

        /// Sample size
        #[clap(short, long, default_value_t=10)]
        size: usize,

        /// Population file name.
        #[clap(name="population's file name")]
        population_fn: Option<String>,
    },

    #[clap(arg_required_else_help = false)]
    /// Weighted reservoir sampling
    Weighted {
        /// Seed for reproducibility
        #[clap(long, parse(try_from_str))]
        seed: Option<u64>,

        /// Sample size
        #[clap(short, long, default_value_t=10)]
        size: usize,

        /// Population file name.
        #[clap(name="population's file name")]
        population_fn: String,

        /// Weights file name.
        #[clap(name="weights' file name")]
        weight_fn: String,
    },
}



// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}



/// Return an open file or stdin if no filename.
fn get_reader(filename: &Option<String>) -> Box<dyn BufRead>
{
    // https://stackoverflow.com/a/49964042
    // https://www.reddit.com/r/rust/comments/jv3q3e/comment/gci1mww/?utm_source=share&utm_medium=web2x&context=3
    match filename {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) if filename == "-"  => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
    }
}



/// Use the seed to seed a SmallRng or else use thread_rng().
fn get_rng(seed: &Option<u64>) -> SmallRng
{
    match seed {
        None => SmallRng::from_rng(thread_rng()).unwrap(),
        Some(s) => SmallRng::seed_from_u64(*s)
    }
}





fn main() {
    let args = Cli::parse();
    match &args.command {
        Commands::Unweighted { size, seed, population_fn } => {
            let mut rng = get_rng(seed);
            let population = get_reader(population_fn);

            let samples = l(
                &mut population.lines().map(|v| v.unwrap()),
                *size,
                &mut rng);
            for sample in samples {
                println!("{}", sample);
            }
        }
        Commands::Weighted { size, seed, weight_fn, population_fn } => {
            let mut rng = get_rng(seed);
            let weights = read_lines(weight_fn)
                .unwrap()
                .map(Result::unwrap)
                .map(|v| v.parse::<f64>().unwrap());
            let values = read_lines(population_fn)
                .unwrap()
                .map(Result::unwrap);
            let mut weighted_samples = weights.zip(values);

            let samples = a_exp_j(
                &mut weighted_samples,
                *size,
                &mut rng);
            for sample in samples {
                println!("{}", sample);
            }
        }
    };
}
