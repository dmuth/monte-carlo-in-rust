

#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rand::Rng;
use rand::rngs::{StdRng, ThreadRng};

use rand::SeedableRng;
use rand::thread_rng;


fn rand_seed(seed: u64) -> StdRng {
    StdRng::seed_from_u64(seed)
}

fn rand_seed_no() -> ThreadRng {
    rand::thread_rng()
}

fn generate_random_number(seed: Option<u64>) -> u32 {

    //let mut rng: Box<dyn Rng> = match seed {
    let mut rng: Box<dyn rand::RngCore> = match seed {
        Some(s) => {
            Box::new(rand_seed(s))
        },
        None => {
            Box::new(rand_seed_no())

        }
    };

    // Generate a random number between 1 and 10
    rng.gen_range(1..=10)
}


fn main() {

    // Create a random number generator
    //let mut rng = rand::thread_rng();

    // Generate a random integer between 1 and 10
    //let random_number = rng.gen_range(1..=10);

    //println!("Random number between 1 and 10: {}", random_number);
    //println!("Random number between 1 and 10: {}", generate_random_number(None) );

    println!("Random number between 1 and 10: {}", generate_random_number(None) );
    println!("Random number between 1 and 10: {}", generate_random_number(None) );
    println!("Random number between 1 and 10: {}", generate_random_number(None) );

    println!("Random number between 1 and 10: {}", generate_random_number(Some(42)) );
    println!("Random number between 1 and 10: {}", generate_random_number(Some(42)) );
    println!("Random number between 1 and 10: {}", generate_random_number(Some(42)) );

    println!("Hello, world!");

} // End of main()


#[cfg(test)]
mod tests {
    use super::*;

/*
    #[test]
    fn test_generate_random_number() {
        // Run the function multiple times to test it
        for _ in 0..100 {
            let random_number = generate_random_number(None);
            // Check if the number is within the expected range
            assert!(random_number >= 1 && random_number <= 10, 
                    "Random number out of range: {}", random_number);
        }
    }
*/

}


