

#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rand::{Rng, SeedableRng, thread_rng};
use rand::rngs::{StdRng, ThreadRng};


/*
* Our structure which holds the rng
*/
struct Random {
    rng: Box<dyn rand::RngCore>,
}


impl Random {
    /*
    * Return our seed, based on either an integer or randomly.
    * We should really only use the integer for unit tests or other
    * reproducable tests.
    */
    fn new(seed: Option<u64>) -> Self {

        let rng: Box<dyn rand::RngCore> = match seed {
            Some(s) => Box::new(StdRng::seed_from_u64(s)),
            None => Box::new(rand::thread_rng()),
        };

        Random { rng }

    }

    /*
    * Return a randon number in our range.
    */
    fn generate(&mut self, lower: u32, upper: u32) -> u32 {
        self.rng.gen_range(lower..=upper)
    }

}



fn main() {

    let mut rng = Random::new(None);

    println!("Random number between 1 and 10: {}", rng.generate(1, 10) );

    println!("Hello, world!");

} // End of main()


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_number() {

        let mut rng = Random::new(None);

        for _ in 0..100 {
            let random_number = rng.generate(1, 10);
            assert!(random_number >= 1 && random_number <= 10, 
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

}


