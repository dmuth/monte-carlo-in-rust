

use monte_carlo::Random;
use monte_carlo::Points;


#[test]
fn test_get_random_points() {

    let mut rng = Random::new(None);
    let size = 10;
    let num_points = 10;

    let _points = Points::new(&mut rng, size, num_points);

    let points = _points.get_points();

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

    let _points = Points::new(&mut rng, size, num_points);

    let points = _points.get_points();

    assert_eq!(points[0].x, 3);
    assert_eq!(points[0].y, 6);

}

