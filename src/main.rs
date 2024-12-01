

//#![allow(unused_imports)]


//use log::{info, warn, error, debug, trace};
use log::{info};

use env_logger::{Builder, Env};
use serde_json::json;
use statrs::statistics::Statistics;


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

    if args.avg_multiple_runs == 0 {

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
                metrics
            });

        info!("Metrics: {:?}", metrics);

        if args.metrics {
            info!("--metrics specified, so we're printing metrics.");
            println!("{}", data.to_string() );

        } else {
            if args.benchmark {
                info!("--benchmark specified, so we're printing a benchmark!");
                let speed = metrics.num_points as f64 / metrics.wallclock.as_secs_f64();
                println!("{:.3}", speed);

            } else {
                println!("{:?}", pi);

            }

        }

    } else {
        info!("Doing {:?} runs and averaging the result..", args.avg_multiple_runs);

        let mut values = Vec::<f64>::new();
        let mut min_decimal_places = u8::MAX;

        for i in 1..=args.avg_multiple_runs {
            info!("Beginning run {:?}/{:?}", i, args.avg_multiple_runs);

            let app = App::new(grid_size, num_points, num_threads, batch_size, cache, turbo, random_seed);
            let (pi, _) = app.go();
            values.push(pi);

            let decimal_places = get_decimal_places(pi);
            if decimal_places < min_decimal_places {
                min_decimal_places = decimal_places;
            }

        }

        let mean = values.clone().mean();
        let std_dev = values.clone().std_dev();
        info!("Pi values: {:?}", values);
        info!("Mean: {:?}, std_dev: {:?}, min_decimal_places: {:?}", 
            mean, std_dev, min_decimal_places);
        println!("{:.1$}", mean, min_decimal_places as usize);

    }


} // End of main()


/*
* Get the number of decimal places in a float.
*/
fn get_decimal_places(num: f64) -> u8 {

    //
    // Turn into a string with many decimal places and then 
    // find the index of the decimal.
    //
    let value_str = format!("{}", num);
    let pos = value_str.find(".");

    match pos {
        Some(pos) => {
            value_str[pos + 1..].len() as u8
        },
        None => { 0 }
    }

}


