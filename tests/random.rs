

use monte_carlo::Random;


#[test]
fn test_generate_random_number() {

    let mut rng = Random::new(None);

    for _ in 0..100 {
        let random_number = rng.generate(0, 9);
        assert!(random_number <= 9, 
            "Random number out of range: {}", random_number);
    }

}

#[test]
fn test_generate_random_number_with_seed() {

    let mut rng = Random::new(Some(12345));

    let random_number = rng.generate(1, 10);
    assert_eq!(random_number, 6);
    let random_number = rng.generate(1, 10);
    assert_eq!(random_number, 4);
    let random_number = rng.generate(1, 10);
    assert_eq!(random_number, 10);

}


#[test]
fn test_generate_random_point() {

    let mut rng = Random::new(None);

    for _ in 0..100 {
        let point = rng.generate_point(0, 9);
        assert!(
            point.x <= 9 && point.y <= 9,
            "Random number out of range: {:?}", point);
    }

}

