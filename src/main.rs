

// Debugging for development
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

//use log::{info, warn, error, debug, trace};
use log::{info};

use std::thread::sleep;
use std::time::Instant;
use std::time::Duration;
use std::collections::HashMap;

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
use grid::Grid;
use points::{Points};
use random::Random;
use timer::Timer;



fn main() {

    // Set default logging level to info.
    Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .init();

    let args = args::parse();
    info!("Args: {:?}", args);
    //sleep(Duration::from_millis(100));

    let grid_size = 10;
    let num_points = 1000;
    let num_threads = 2;
    let batch_size = 100;
    let random_seed = Some(12345);
    let turbo = false;
    let app = App::new(grid_size, num_points, num_threads, batch_size, turbo, random_seed);

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


    println!("Pi: {:?}", pi);
    println!("Metrics: {:?}", metrics);
    println!("JSON: {}", data.to_string() );

} // End of main()



