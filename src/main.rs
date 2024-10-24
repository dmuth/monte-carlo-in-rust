

// Debugging for development
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod random;
use random::Random;


fn main() {

    let mut rng = Random::new(None);

    println!("Random number between 1 and 10: {:?}", rng.get(1, 10) );
    let point = rng.get_point(0, 10);
    println!("Random point: {}, {}", point.x, point.y );

    println!("Hello, world!");

} // End of main()



