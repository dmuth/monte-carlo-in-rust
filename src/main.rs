

//#![allow(unused_imports)]


//use log::{info, warn, error, debug, trace};
use log::{info};

use env_logger::{Builder, Env};
use serde_json::json;

mod args;
mod app;
mod cache;
mod grid;
mod metric;
mod metrics;
mod point;
mod points;
mod random;
mod timer;

use app::App;


fn main() {

    // Set default logging level to info.
    Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .init();

    let args = args::parse();
    info!("Args: {:?}", args);
    //sleep(Duration::from_millis(100));

    let grid_size = args.grid_size;
    let num_points = args.count;
    let num_threads = args.num_threads;
    let batch_size = args.batch_size;
    //let random_seed = Some(12345);
    let random_seed = args.random_seed;
    let turbo = args.turbo;
    let cache = args.cache;
    let app = App::new(grid_size, num_points, num_threads, batch_size, cache, turbo, random_seed);

    let (pi, metrics) = app.go();

    let data = json!({
        "pi": pi,
        "grid_size": grid_size,
        "num_points": num_points,
        "num_threads": num_threads,
        "batch_size": batch_size,
        "random_seed": random_seed,
        "turbo": turbo,
        "metrics": 
            serde_json::to_string(&metrics).expect("Unable to serialize metrics")
        });

    info!("Metrics: {:?}", metrics);

    if args.metrics {
        println!("{}", data.to_string() );
    } else {
        println!("{:?}", pi);
    }

} // End of main()



