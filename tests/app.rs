

use monte_carlo::App;
use monte_carlo::Random;


#[test]
fn test_app() {

    let mut rng = Random::new(Some(12345));

    let grid_size = 10;
    let num_points = 10000;
    let num_threads = 1;
    let batch_size = 1000;
    let app = App::new(&mut rng, grid_size, num_points, num_threads, batch_size, false);

    let pi = app.go();
    assert_eq!(pi, 2.9784);

}

#[test]
fn test_app_turbo() {

    let mut rng = Random::new(Some(12345));

    let grid_size = 10;
    let num_points = 10000;
    let num_threads = 1;
    let batch_size = 1000;
    let app = App::new(&mut rng, grid_size, num_points, num_threads, batch_size, true);

    let pi = app.go();
    assert_eq!(pi, 2.9784);

}

#[test]
fn test_app_multithreading() {

    let mut rng = Random::new(Some(12345));

    let grid_size = 10;
    let num_points = 1000;
    let num_threads = 2;
    let batch_size = 100;
    let app = App::new(&mut rng, grid_size, num_points, num_threads, batch_size, false);

    let tmp = app.go();
    println!("TEST: {:?}", tmp);

}

#[test]
fn test_app_get_batch_count() {

    let mut rng = Random::new(Some(12345));

    let grid_size = 10;
    let num_points = 1000;
    let num_threads = 1;
    let batch_size = 100;
    let mut app = App::new(&mut rng, grid_size, num_points, num_threads, batch_size, false);
    assert_eq!(app.get_batch_count(), 100);
    assert_eq!(app.get_batch_count(), 100);

    let num_points = 100;
    let mut app = App::new(&mut rng, grid_size, num_points, num_threads, batch_size, false);
    assert_eq!(app.get_batch_count(), 100);
    assert_eq!(app.get_batch_count(), 0);
    assert_eq!(app.get_batch_count(), 0);

    let num_points = 150;
    let mut app = App::new(&mut rng, grid_size, num_points, num_threads, batch_size, false);
    assert_eq!(app.get_batch_count(), 100);
    assert_eq!(app.get_batch_count(), 50);
    assert_eq!(app.get_batch_count(), 0);
    assert_eq!(app.get_batch_count(), 0);

}

