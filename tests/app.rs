

use monte_carlo::App;


#[test]
fn test_app() {

    let grid_size = 10;
    let num_points = 10000;
    let num_threads = 1;
    let batch_size = 1000;
    let random_seed = Some(12345);
    let app = App::new(grid_size, num_points, num_threads, batch_size, false, random_seed);

    let (pi, _metrics) = app.go();
    assert_eq!(pi, 2.908);

}

#[test]
fn test_app_turbo() {

    let grid_size = 10;
    let num_points = 10000;
    let num_threads = 1;
    let batch_size = 1000;
    let random_seed = Some(12345);
    let app = App::new(grid_size, num_points, num_threads, batch_size, true, random_seed);

    let (pi, _metrics) = app.go();
    assert_eq!(pi, 2.908);

}

#[test]
fn test_app_multithreading() {

    let grid_size = 10;
    let num_points = 1000;
    let num_threads = 2;
    let batch_size = 100;
    let random_seed = Some(12345);
    let app = App::new(grid_size, num_points, num_threads, batch_size, false, random_seed);

    let (pi, _metrics) = app.go();
    assert_eq!(pi, 2.968);

}

#[test]
fn test_app_get_batch_count() {

    let grid_size = 10;
    let num_points = 1000;
    let num_threads = 1;
    let batch_size = 100;
    let random_seed = Some(12345);
    let mut app = App::new(grid_size, num_points, num_threads, batch_size, false, random_seed);
    assert_eq!(app.get_batch_count(), 100);
    assert_eq!(app.get_batch_count(), 100);

    let num_points = 100;
    let mut app = App::new(grid_size, num_points, num_threads, batch_size, false, random_seed);
    assert_eq!(app.get_batch_count(), 100);
    assert_eq!(app.get_batch_count(), 0);
    assert_eq!(app.get_batch_count(), 0);

    let num_points = 150;
    let mut app = App::new(grid_size, num_points, num_threads, batch_size, false, random_seed);
    assert_eq!(app.get_batch_count(), 100);
    assert_eq!(app.get_batch_count(), 50);
    assert_eq!(app.get_batch_count(), 0);
    assert_eq!(app.get_batch_count(), 0);

}

