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
use rand::thread_rng;
use std::fs::File;
use std::io::{
    BufRead,
    self,
    stdin,
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
    #[clap(arg_required_else_help = false)]
    Unweighted {
        #[clap(short, long, default_value_t = 10)]
        size: usize,
    },

    #[clap(arg_required_else_help = false)]
    Weighted {
        #[clap(short, long, default_value_t = 10)]
        size: usize,

        #[clap(name="values' file name")]
        value_fn: String,

        #[clap(name="weights' file name")]
        weight_fn: String,
    },
}



// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}






fn main() {
    let args = Cli::parse();
    match &args.command {
        Commands::Unweighted { size } => {
            let mut rng = thread_rng();
            let input = stdin();
            let mut input_lines = input.lock().lines().map(|r| r.unwrap());
            let samples = l(
                &mut input_lines,
                *size,
                &mut rng);
            for sample in samples {
                println!("{}", sample);
            }
        }
        Commands::Weighted { size, weight_fn, value_fn } => {
            let mut rng = thread_rng();
            let weights = read_lines(weight_fn)
                .unwrap()
                .map(Result::unwrap)
                .map(|v| v.parse::<f64>().unwrap());
            let values = read_lines(value_fn)
                .unwrap()
                .map(Result::unwrap);
            let samples = a_exp_j(
                &mut weights.zip(values),
                *size,
                &mut rng);
            for sample in samples {
                println!("{}", sample);
            }
        }
    };
}
