
use rand::{Rng, SeedableRng};
use rand::rngs::{StdRng};

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
    pub fn generate(&mut self, lower: u32, upper: u32) -> u32 {
        self.rng.gen_range(lower..=upper)
    }

}



