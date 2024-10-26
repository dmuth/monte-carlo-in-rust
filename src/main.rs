

// Debugging for development
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

//use log::{info, warn, error, debug, trace};
use log::{info};

use std::thread::sleep;
use std::time::Instant;
use std::time::Duration;

use env_logger::{Builder, Env};

mod args;
mod grid;
mod points;
mod random;
mod timer;

use grid::Grid;
use points::{Points};
use random::Random;
use timer::Timer;



fn main() {

    let mut timer = Timer::new();
    println!("TEST elapsed: {:?}, {:?}", timer.get_elapsed(), timer.get_elapsed_time_t().unwrap() );
    sleep(Duration::from_millis(100));
    println!("TEST elapsed: {:?}, {:?}", timer.get_elapsed(), timer.get_elapsed_time_t().unwrap() );

    let start_time = Instant::now();

    // Set default logging level to info.
    Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .init();

    let args = args::parse();
    info!("Args: {:?}", args);
    //sleep(Duration::from_millis(100));

    let mut rng = Random::new(None);

    let points = Points::new(&mut rng, 10, 10);
    println!("TEST POINTS: {:?}", points);


    println!("Random number between 1 and 10: {:?}", rng.get(1, 10) );
    let point = rng.get_point(10);
    println!("Random point: {}, {}", point.x, point.y );

    info!("Time elapsed: {:?}", start_time.elapsed());

    let grid = Grid::new(10);
    println!("GRID: {:?}", grid);


    println!("Hello, world!");

} // End of main()



