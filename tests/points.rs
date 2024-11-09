

use monte_carlo::Random;
use monte_carlo::Point;
use monte_carlo::Points;
use monte_carlo::CircleMode;


#[test]
fn test_get_random_points() {

    let mut rng = Random::new(None);
    let grid_size = 10;
    let num_points = 10;

    let points = Points::new(&mut rng, grid_size, num_points);

    let points = points.get_points();

    for i in 0..num_points {
        let point = &points[i as usize];
        assert!(
            point.x <= 10 && point.y <= 10,
            "Random number out of range: {:?}", point);
    }

}

#[test]
fn test_get_random_points_with_seed() {

    let mut rng = Random::new(Some(12345));
    let grid_size = 10;
    let num_points = 10;

    let points = Points::new(&mut rng, grid_size, num_points);

    let points = points.get_points();

    assert_eq!(points[0].x, 3);
    assert_eq!(points[0].y, 6);

}

#[test]
fn test_calculate_points_in_circle_with_seed() {

    let mut rng = Random::new(Some(12345));
    let grid_size = 10;
    let num_points = 10;

    let points = Points::new(&mut rng, grid_size, num_points);

    let num_points = points.get_points_in_circle();
    assert_eq!(num_points, 9);

}

#[test]
fn test_calculate_points_in_circle_turbo_with_seed() {

    let mut rng = Random::new(Some(12345));
    let grid_size = 10;
    let num_points = 10;

    let points = Points::new(&mut rng, grid_size, num_points);

    let num_points = points.get_points_in_circle_turbo();
    assert_eq!(num_points, 9);

}

#[test]
fn test_calculate_points_in_circle_manually() {

    let helper = |x: u64, y:u64 | -> Points {

        let mut points = Vec::<Point>::new();
        let point = Point{x:x, y:y};
        points.push(point);

        return Points::new_with_points(10, points);

    };

    let points = helper(1,1);
    assert_eq!(1, points._get_points_in_circle(None));

    let points = helper(5, 5);
    assert_eq!(1, points._get_points_in_circle(None));

    let points = helper(0, 0);
    assert_eq!(1, points._get_points_in_circle(None));

    let points = helper(0, 10);
    assert_eq!(1, points._get_points_in_circle(None));

    let points = helper(10, 0);
    assert_eq!(1, points._get_points_in_circle(None));

    let points = helper(10, 1);
    assert_eq!(0, points._get_points_in_circle(None));

    let points = helper(1, 10);
    assert_eq!(0, points._get_points_in_circle(None));

} 

#[test]
fn test_calculate_points_in_circle_manually_turbo() {

    let helper = |x: u64, y:u64 | -> Points {

        let mut points = Vec::<Point>::new();
        let point = Point{x:x, y:y};
        points.push(point);

        return Points::new_with_points(10, points);

    };

    let points = helper(1,1);
    assert_eq!(1, points._get_points_in_circle(Some(CircleMode::Turbo)));

    let points = helper(5, 5);
    assert_eq!(1, points._get_points_in_circle(Some(CircleMode::Turbo)));

    let points = helper(0, 0);
    assert_eq!(1, points._get_points_in_circle(Some(CircleMode::Turbo)));

    let points = helper(0, 10);
    assert_eq!(1, points._get_points_in_circle(Some(CircleMode::Turbo)));

    let points = helper(10, 0);
    assert_eq!(1, points._get_points_in_circle(Some(CircleMode::Turbo)));

    let points = helper(10, 1);
    assert_eq!(0, points._get_points_in_circle(Some(CircleMode::Turbo)));

    let points = helper(1, 10);
    assert_eq!(0, points._get_points_in_circle(Some(CircleMode::Turbo)));

} 



