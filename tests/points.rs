

use std::rc::Rc;
use std::cell::RefCell;

use monte_carlo::Random;
use monte_carlo::Point;
use monte_carlo::Points;
use monte_carlo::Cache;
use monte_carlo::CircleMode;


#[test]
fn test_points_get_random_points() {

    let mut rng = Random::new(None);
    let grid_size = 10;
    let num_points = 10;

    let points = Points::new(&mut rng, grid_size, num_points, None);

    let points = points.get_points();

    for i in 0..num_points {
        let point = &points[i as usize];
        assert!(
            point.x <= 10 && point.y <= 10,
            "Random number out of range: {:?}", point);
    }

}

#[test]
fn test_points_get_random_points_with_seed() {

    let mut rng = Random::new(Some(12345));
    let grid_size = 10;
    let num_points = 10;

    let points = Points::new(&mut rng, grid_size, num_points, None);

    let points = points.get_points();

    assert_eq!(points[0].x, 3);
    assert_eq!(points[0].y, 6);

}

#[test]
fn test_points_calculate_points_in_circle_with_seed() {

    let mut rng = Random::new(Some(12345));
    let grid_size = 10;
    let num_points = 10;

    let points = Points::new(&mut rng, grid_size, num_points, None);

    let num_points = points.get_points_in_circle();
    assert_eq!(num_points, 9);

}

#[test]
fn test_points_calculate_points_in_circle_turbo_with_seed() {

    let mut rng = Random::new(Some(12345));
    let grid_size = 10;
    let num_points = 10;

    let points = Points::new(&mut rng, grid_size, num_points, None);

    let num_points = points.get_points_in_circle_turbo();
    assert_eq!(num_points, 9);

}

#[test]
fn test_points_calculate_points_in_circle_with_seed_and_cache() {

    let mut rng = Random::new(Some(12345));
    let grid_size = 10;
    let num_points = 10;
    let cache = Rc::new(RefCell::new(Cache::new(grid_size)));
    
    let points = Points::new(&mut rng, grid_size, num_points, Some(cache));

    let num_points = points.get_points_in_circle();
    assert_eq!(num_points, 9);

}

#[test]
fn test_points_calculate_points_in_circle_turbo_with_seed_and_cache() {

    let mut rng = Random::new(Some(12345));
    let grid_size = 10;
    let num_points = 10;
    let cache = Rc::new(RefCell::new(Cache::new(grid_size)));

    let points = Points::new(&mut rng, grid_size, num_points, Some(cache));

    let num_points = points.get_points_in_circle_turbo();
    assert_eq!(num_points, 9);

}

#[test]
fn test_points_calculate_points_in_circle_manually() {

    let helper = |x: u64, y:u64 | -> Points {

        let mut points = Vec::<Point>::new();
        let point = Point{x:x, y:y};
        points.push(point);

        return Points::new_with_points(10, points, None);

    };

    let points = helper(1,1);
    let points_in_circle = points._get_points_in_circle(None);
    assert_eq!(1, points_in_circle);

    let points = helper(5, 5);
    let points_in_circle = points._get_points_in_circle(None);
    assert_eq!(1, points_in_circle);

    let points = helper(0, 0);
    let points_in_circle = points._get_points_in_circle(None);
    assert_eq!(1, points_in_circle);

    let points = helper(0, 10);
    let points_in_circle = points._get_points_in_circle(None);
    assert_eq!(1, points_in_circle);

    let points = helper(10, 0);
    let points_in_circle = points._get_points_in_circle(None);
    assert_eq!(1, points_in_circle);

    let points = helper(10, 1);
    let points_in_circle = points._get_points_in_circle(None);
    assert_eq!(0, points_in_circle);

    let points = helper(1, 10);
    let points_in_circle = points._get_points_in_circle(None);
    assert_eq!(0, points_in_circle);

} 

#[test]
fn test_points_calculate_points_in_circle_manually_turbo() {

    let helper = |x: u64, y:u64 | -> Points {

        let mut points = Vec::<Point>::new();
        let point = Point{x:x, y:y};
        points.push(point);

        return Points::new_with_points(10, points, None);

    };

    let points = helper(1,1);
    let points_in_circle = points._get_points_in_circle(Some(CircleMode::Turbo));
    assert_eq!(1, points_in_circle);

    let points = helper(5, 5);
    let points_in_circle = points._get_points_in_circle(Some(CircleMode::Turbo));
    assert_eq!(1, points_in_circle);

    let points = helper(0, 0);
    let points_in_circle = points._get_points_in_circle(Some(CircleMode::Turbo));
    assert_eq!(1, points_in_circle);

    let points = helper(0, 10);
    let points_in_circle = points._get_points_in_circle(Some(CircleMode::Turbo));
    assert_eq!(1, points_in_circle);

    let points = helper(10, 0);
    let points_in_circle = points._get_points_in_circle(Some(CircleMode::Turbo));
    assert_eq!(1, points_in_circle);

    let points = helper(10, 1);
    let points_in_circle = points._get_points_in_circle(Some(CircleMode::Turbo));
    assert_eq!(0, points_in_circle);

    let points = helper(1, 10);
    let points_in_circle = points._get_points_in_circle(Some(CircleMode::Turbo));
    assert_eq!(0, points_in_circle);

} 



