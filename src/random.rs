
use rand::{Rng, SeedableRng};
use rand::rngs::{StdRng};

/*
* Our structure which holds the rng
*/
pub struct Random {
    rng: Box<dyn rand::RngCore>,
}


/*
* Our point.
*/
#[derive(Debug)]
pub struct Point {
    pub x: u64,
    pub y: u64
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
    pub fn get_point(&mut self, lower: u64, upper: u64) -> Point {

        Point {
            x: self.rng.gen_range(lower..=upper),
            y: self.rng.gen_range(lower..=upper),
        }

    }

}



