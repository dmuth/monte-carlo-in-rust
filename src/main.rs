

// Debugging for development
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

//use log::{info, warn, error, debug, trace};
use log::{info};

//use std::thread::sleep;
//use std::time::Duration
use std::time::Instant;

use env_logger::{Builder, Env};


mod args;
mod random;
use random::Random;


fn main() {

    let start_time = Instant::now();

    // Set default logging level to info.
    Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .init();

    let args = args::parse();
    info!("Args: {:?}", args);
    //sleep(Duration::from_millis(100));

    let mut rng = Random::new(None);

    println!("Random number between 1 and 10: {:?}", rng.get(1, 10) );
    let point = rng.get_point(0, 10);
    println!("Random point: {}, {}", point.x, point.y );

    info!("Time elapsed: {:?}", start_time.elapsed());

    println!("Hello, world!");

} // End of main()



