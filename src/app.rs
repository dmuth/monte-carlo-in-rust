
// Debugging
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use std::fmt;

use crate::random::Random;

/*
* Our structure.
*/
pub struct App<'a> {
    rng: &'a mut Random,
    size: u64,
    num_points: u64,
    num_threads: u64,
    batch_size: u64,
    turbo: bool,
}

/*
* Our custom formatted since we can't print out the random value.
*/
impl fmt::Debug for App<'_> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("App")
            .field("random", &"<Random>") // Custom format for `Random`
            .field("size", &self.size)
            .field("num_points", &self.num_points)
            .field("num_threads", &self.num_threads)
            .field("batch_size", &self.batch_size)
            .field("turbo", &self.turbo)
            .finish()
    }

}

impl<'a> App<'a> {

    pub fn new(rng: &'a mut Random, size: u64, num_points: u64, 
        num_threads: u64, batch_size: u64, turbo: bool) -> Self {

        App {
            rng: rng,
            size: size,
            num_points: num_points,
            num_threads: num_threads,
            batch_size: batch_size,
            turbo: turbo,
            }

    }

    pub fn go(self: App<'a>) {

        if self.num_threads > 1 {
            panic!("Thead count > 1 currently not supported!");
        }
         
println!("TEST: {:?}", self);
/* TEST/TODO:
- Algorithm to go through points and update grid
    - Maybe put in a function to get a number equal to the batch size or the number of points left
- Return value (and metrics: time taken, params, turbo, size, num_points)
*/

    }

    pub fn get_batch_count(&mut self) -> u64 {

        if self.num_points < self.batch_size {
            let retval = self.num_points;
            self.num_points = 0;
            return retval;
        }

        self.num_points -= self.batch_size;
        return self.batch_size;

    }

}

