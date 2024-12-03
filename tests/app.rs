
use std::ops::Sub;

use monte_carlo::App;
use monte_carlo::Progress;


/*
* Helper function to verify that values are within a certain range.
*/
fn within_tolerance<T>(a: T, b: T, tolerance: T) -> bool
where
    //
    // Ensure that the type supports subtraction, returns the same type, 
    // supports comparison, and copy operations.
    //
    T: Sub<Output = T> + PartialOrd + Copy {
    let diff = if a > b { a - b } else { b - a };
    diff <= tolerance
}


#[test]
fn test_app() {

    let grid_size = 10;
    let num_points = 10000;
    let num_threads = 1;
    let batch_size = 1000;
    let random_seed = Some(12345);
    let app = App::new(grid_size, num_points, num_threads, batch_size, false, false, false, random_seed);

    let (pi, metrics) = app.go();
    assert_eq!(pi, 2.908);
    assert_eq!(metrics.cache_hits, 0);
    assert_eq!(metrics.cache_misses, 0);

}


#[test]
fn test_app_cache() {

    let grid_size = 10;
    let num_points = 1000;
    let num_threads = 1;
    let batch_size = 1000;
    let random_seed = Some(12345);
    let cache = true;
    let cache_precompute = false;
    let turbo = false;

    let app = App::new(grid_size, num_points, num_threads, batch_size, cache, cache_precompute, turbo, random_seed);
    let (pi, metrics) = app.go();
    assert_eq!(pi, 2.908);
    assert_eq!(metrics.cache_hits, 989);
    assert_eq!(metrics.cache_misses, 11);

    let num_points = 1001;
    let app = App::new(grid_size, num_points, num_threads, batch_size, cache, cache_precompute, turbo, random_seed);
    let (pi, metrics) = app.go();
    assert_eq!(pi, 2.909090909090909);
    assert_eq!(metrics.cache_hits, 990);
    assert_eq!(metrics.cache_misses, 11);

}


#[test]
fn test_app_turbo() {

    let grid_size = 10;
    let num_points = 10000;
    let num_threads = 1;
    let batch_size = 1000;
    let cache = false;
    let cache_precompute = false;
    let turbo = true;
    let random_seed = Some(12345);
    let app = App::new(grid_size, num_points, num_threads, batch_size, cache, cache_precompute, turbo, random_seed);

    let (pi, metrics) = app.go();
    assert_eq!(pi, 2.908);
    assert_eq!(metrics.cache_hits, 0);
    assert_eq!(metrics.cache_misses, 0);

}


#[test]
fn test_app_cache_turbo() {

    let grid_size = 10;
    let num_points = 10000;
    let num_threads = 1;
    let batch_size = 1000;
    let cache = true;
    let cache_precompute = false;
    let turbo = true;
    let random_seed = Some(12345);
    let app = App::new(grid_size, num_points, num_threads, batch_size, cache, cache_precompute, turbo, random_seed);

    let (pi, metrics) = app.go();
    assert_eq!(pi, 2.908);
    assert_eq!(metrics.cache_hits, 9989);
    assert_eq!(metrics.cache_misses, 11);

}


#[test]
fn test_app_multithreading() {

    let grid_size = 10;
    let num_points = 1000;
    let num_threads = 2;
    let batch_size = 100;
    let cache = false;
    let cache_precompute = false;
    let turbo = false;
    let random_seed = Some(12345);
    let app = App::new(grid_size, num_points, num_threads, batch_size, cache, cache_precompute, turbo, random_seed);

    let (pi, metrics) = app.go();
    assert!(within_tolerance(2.968, pi, 0.1), "Value of pi outside tolerance");
    assert_eq!(metrics.cache_hits, 0);
    assert_eq!(metrics.cache_misses, 0);

}


#[test]
fn test_app_multithreading_cache() {

    let grid_size = 10;
    let num_points = 1000;
    let num_threads = 2;
    let batch_size = 100;
    let cache = true;
    let cache_precompute = false;
    let turbo = false;
    let random_seed = Some(12345);
    let app = App::new(grid_size, num_points, num_threads, batch_size, cache, cache_precompute, turbo, random_seed);

    let (pi, metrics) = app.go();
    assert!(within_tolerance(2.908, pi, 0.1), "Value of pi outside tolerance");
    assert!(within_tolerance(metrics.cache_hits, 978, 20));
    assert!(within_tolerance(metrics.cache_misses, 22, 20));

}


#[test]
fn test_app_multithreading_turbo() {

    let grid_size = 10;
    let num_points = 1000;
    let num_threads = 2;
    let batch_size = 100;
    let cache = false;
    let cache_precompute = false;
    let turbo = true;
    let random_seed = Some(12345);
    let app = App::new(grid_size, num_points, num_threads, batch_size, cache, cache_precompute, turbo, random_seed);

    let (pi, metrics) = app.go();
    assert!(within_tolerance(2.908, pi, 0.1), "Value of pi outside tolerance");
    assert_eq!(metrics.cache_hits, 0);
    assert_eq!(metrics.cache_misses, 0);

}


#[test]
fn test_app_multithreading_cache_turbo() {

    let grid_size = 10;
    let num_points = 1000;
    let num_threads = 2;
    let batch_size = 100;
    let cache = true;
    let cache_precompute = false;
    let turbo = true;
    let random_seed = Some(12345);
    let app = App::new(grid_size, num_points, num_threads, batch_size, cache, cache_precompute, turbo, random_seed);

    let (pi, metrics) = app.go();
    assert!(within_tolerance(2.908, pi, 0.1), "Value of pi outside tolerance");
    assert!(within_tolerance(metrics.cache_hits, 978, 20));
    assert!(within_tolerance(metrics.cache_misses, 22, 20));

}


#[test]
fn test_app_get_batch_count() {

    let grid_size = 10;
    let num_points = 1000;
    let num_threads = 1;
    let batch_size = 100;
    let random_seed = Some(12345);
    let mut app = App::new(grid_size, num_points, num_threads, batch_size, false, false, false, random_seed);
    assert_eq!(app.get_batch_count(), 100);
    assert_eq!(app.get_batch_count(), 100);

    let num_points = 100;
    let mut app = App::new(grid_size, num_points, num_threads, batch_size, true, false, false, random_seed);
    assert_eq!(app.get_batch_count(), 100);
    assert_eq!(app.get_batch_count(), 0);
    assert_eq!(app.get_batch_count(), 0);

    let num_points = 150;
    let mut app = App::new(grid_size, num_points, num_threads, batch_size, false, true, false, random_seed);
    assert_eq!(app.get_batch_count(), 100);
    assert_eq!(app.get_batch_count(), 50);
    assert_eq!(app.get_batch_count(), 0);
    assert_eq!(app.get_batch_count(), 0);

}


#[test]
fn test_app_progress() {

    let num_points = 100;
    let mut progress = Progress::new(num_points);
    assert_eq!(progress.get_percent(1), 1);
    assert_eq!(progress.get_percent(5), 5);
    assert_eq!(progress.display(1), true);
    assert_eq!(progress.display(1), false);
    assert_eq!(progress.display(1), false);
    assert_eq!(progress.display(5), true);
    assert_eq!(progress.display(5), false);

    let num_points = 1000;
    let mut progress = Progress::new(num_points);
    assert_eq!(progress.get_percent(1), 0);
    assert_eq!(progress.get_percent(5), 0);
    assert_eq!(progress.get_percent(10), 1);
    assert_eq!(progress.get_percent(10), 1);
    assert_eq!(progress.get_percent(20), 2);
    assert_eq!(progress.get_percent(21), 2);
    assert_eq!(progress.display(1), false);
    assert_eq!(progress.display(5), false);
    assert_eq!(progress.display(10), true);
    assert_eq!(progress.display(11), false);
    assert_eq!(progress.display(21), true);
    assert_eq!(progress.display(21), false);

}


