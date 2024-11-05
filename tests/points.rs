

use monte_carlo::Random;
use monte_carlo::Points;


#[test]
fn test_get_random_points() {

    let mut rng = Random::new(None);
    let size = 10;
    let num_points = 10;

    let points = Points::new(&mut rng, size, num_points);

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
    let size = 10;
    let num_points = 10;

    let points = Points::new(&mut rng, size, num_points);

    let points = points.get_points();

    assert_eq!(points[0].x, 3);
    assert_eq!(points[0].y, 6);

}

#[test]
fn test_calculate_points_in_circle_with_seed() {

    let mut rng = Random::new(Some(12345));
    let size = 10;
    let num_points = 10;

    let points = Points::new(&mut rng, size, num_points);

    let num_points = points.get_points_in_circle();
    assert_eq!(num_points, 9);

}

#[test]
fn test_calculate_points_in_circle_turbo_with_seed() {

    let mut rng = Random::new(Some(12345));
    let size = 10;
    let num_points = 10;

    let points = Points::new(&mut rng, size, num_points);

    let num_points = points.get_points_in_circle_turbo();
    assert_eq!(num_points, 9);

}


