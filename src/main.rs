

// Debugging for development
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use monte_carlo::Random;

fn main() {

    let mut rng = Random::new(None);

    println!("Random number between 1 and 10: {}", rng.generate(1, 10) );

    println!("Hello, world!");

} // End of main()



