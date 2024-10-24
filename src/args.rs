

use clap::Parser;
use num_cpus;

#[derive(Parser, Debug)]
#[command(name = "Monte Carlo Simulation to Calculate Pi")]
#[command(author = "Douglas Muth <doug.muth@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Calculate Pi using a Monte Carlo simulation.", long_about = None)]
pub struct Args {

    #[arg(short, long, default_value_t = 10, help = "The Size of the grid.")]
    size: u32,

    #[arg(short, long, default_value_t = 100, help = "How many random numbers do we want to generate in total?")]
    count: u32,

    #[arg(long="cpf", default_value_t = 100, help = "How many random numbers to generate per function invocation?")]
    count_per_function: u32,

    #[arg(long, default_value_t = 1, help = "How many threads to use for computation?")]
    num_threads: u32,

    #[arg(short, long, help = "ADVANCED: Seed the random number generator for deterministic behavior.")]
    random_seed: Option<u32>,

    // TODO: Caching options

}

pub fn parse() -> Args {

    let args = Args::parse();

    if args.size < 10 {
        panic!("Size must be at least 10!");
    }

    if args.num_threads > num_cpus::get().try_into().unwrap() {
        panic!("Number of threads cannot exceed number of cores ({})", num_cpus::get());
    }

    args

} // End of parse()


