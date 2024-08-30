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

    /// Seed for reproducibility
    #[clap(long, parse(try_from_str))]
    seed: Option<u64>,

    /// Sample size
    #[clap(short, long, default_value_t=10)]
    size: usize,
}



#[derive(Subcommand)]
enum Commands {
    #[clap(arg_required_else_help=false, visible_alias="uw")]
    /// Unweighted resevoir sampling
    Unweighted {
        /// Population file name.
        #[clap(name="population's file name")]
        population_fn: Option<String>,
    },

    #[clap(arg_required_else_help=false, visible_alias="w")]
    /// Weighted reservoir sampling
    Weighted {
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





fn iter_nth() {
    let mut iter = 0usize..100;
    loop {
        match iter.nth(1) {
            Some(n) => {
                println!("{}", n);
            }
            None => break,
        }
    }
}

fn main() {
    //iter_nth();
    let args = Cli::parse();
    let mut rng = get_rng(&args.seed);
    match &args.command {
        Commands::Unweighted { population_fn } => {
            let population = get_reader(population_fn);

            let samples = l(
                &mut population.lines().map(|v| v.unwrap()),
                args.size,
                &mut rng);
            for sample in samples {
                println!("{}", sample);
            }
        }
        Commands::Weighted { weight_fn, population_fn } => {
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
                args.size,
                &mut rng);
            for sample in samples {
                println!("{}", sample);
            }
        }
    };
}
