

use clap::Parser;
//use log::{info, warn, error, debug, trace};
use num_cpus;

#[derive(Parser, Debug)]
#[command(name = "Monte Carlo Simulation to Calculate Pi")]
#[command(author = "Douglas Muth <doug.muth@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Calculate Pi using a Monte Carlo simulation.", long_about = None)]
pub struct Args {

    #[arg(short, long, default_value_t = 10, value_name = "num_points", help = "The Size of the grid.")]
    pub grid_size: u64,

    #[arg(short, long, default_value_t = 100, value_name = "count",
        help = "How many random numbers do we want to generate in total?")]
    pub count: u64,

    #[arg(short, long, default_value_t = 100, value_name = "size",
        help = "How many random numbers to generate per function invocation?")]
    pub batch_size: u64,

    #[arg(short, long, default_value_t = 1, value_name = "n",
        help = "How many threads to use for computation?")]
    pub num_threads: u64,

    #[arg(short, long, default_value_t = false,
         help = "Set if you want metrics printed out in JSON format.")]
    pub metrics: bool,

    #[arg(short, long, default_value_t = false, 
        help = "Set to use \"turbo\" mode where a simplified version of the Pythagorean Theorem is used.")]
    pub turbo: bool,

    #[arg(long, default_value_t = false, 
        help = "Set if you want to use caching for circle calculations.")]
    pub cache: bool,

    #[arg(long, default_value_t = false, 
        help = "Precompute the contents of the cache for each thread.  Implies --cache.")]
    pub cache_precompute: bool,

    #[arg(long, default_value_t = false, 
        help = "Benchmark mode.  This will print how many points per second were plotted.")]
    pub benchmark: bool,

    #[arg(short, long, default_value_t = 0, value_name = "n",
        help = "Perform multiple runs and average the values of Pi from all runs.")]
    pub avg_multiple_runs: u64,

    #[arg(short, long, value_name = "seed",
        help = "ADVANCED: Seed the random number generator for deterministic behavior.")]
    pub random_seed: Option<u64>,

}


pub fn parse() -> Args {

    let mut args = Args::parse();

    if args.grid_size < 10 {
        panic!("Size must be at least 10!");
    }

    if args.num_threads > num_cpus::get().try_into().unwrap() {
        panic!("Number of threads cannot exceed number of cores ({})", num_cpus::get());
    }

    if args.cache_precompute {
        args.cache = true;
    }

    args

} // End of parse()


