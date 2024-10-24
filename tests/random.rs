

use monte_carlo::Random;


#[test]
fn test_get_random_number() {

    let mut rng = Random::new(None);

    for _ in 0..100 {
        let random_number = rng.get(0, 9);
        assert!(random_number <= 9, 
            "Random number out of range: {}", random_number);
    }

}

#[test]
fn test_get_random_number_with_seed() {

    let mut rng = Random::new(Some(12345));

    let random_number = rng.get(1, 10);
    assert_eq!(random_number, 4);
    let random_number = rng.get(1, 10);
    assert_eq!(random_number, 6);
    let random_number = rng.get(1, 10);
    assert_eq!(random_number, 5);

}


#[test]
fn test_get_random_point() {

    let mut rng = Random::new(None);

    for _ in 0..100 {
        let point = rng.get_point(0, 9);
        assert!(
            point.x <= 9 && point.y <= 9,
            "Random number out of range: {:?}", point);
    }

}


#[test]
fn test_get_random_point_with_seed() {

    let mut rng = Random::new(Some(12345));

    let point = rng.get_point(0, 9);
    assert_eq!(point.x, 3);
    assert_eq!(point.y, 5);

    let point = rng.get_point(0, 9);
    assert_eq!(point.x, 4);
    assert_eq!(point.y, 6);

}


