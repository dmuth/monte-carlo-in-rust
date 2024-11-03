
use rand::{Rng, SeedableRng};
use rand::rngs::{StdRng};

use crate::point::Point;


/*
* Our structure which holds the rng
*/
pub struct Random {
    rng: Box<dyn rand::RngCore>,
}


impl Random {
    /*
    * Return our seed, based on either an integer or randomly.
    * We should really only use the integer for unit tests or other
    * reproducable tests.
    */
    pub fn new(seed: Option<u64>) -> Self {

        let rng: Box<dyn rand::RngCore> = match seed {
            Some(s) => Box::new(StdRng::seed_from_u64(s)),
            None => Box::new(rand::thread_rng()),
        };

        Random { rng }

    }

    /*
    * Return a randon number in our range.
    */
    pub fn get(&mut self, lower: u64, upper: u64) -> u64 {
        self.rng.gen_range(lower..=upper)
    }

    /**
    * Return a Point with random values.
    */
    pub fn get_point(&mut self, size: u64) -> Point {

        Point {
            x: self.rng.gen_range(0..=size),
            y: self.rng.gen_range(0..=size),
        }

    }

}



