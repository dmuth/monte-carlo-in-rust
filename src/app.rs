
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
            .field("turbo", &self.turbo)
            .finish()
    }

}

impl<'a> App<'a> {

    pub fn new(rng: &'a mut Random, size: u64, num_points: u64, num_threads: u64, turbo: bool) -> Self {

        App {
            rng: rng,
            size: size,
            num_points: num_points,
            num_threads: num_threads,
            turbo: turbo,
            }

    }

    pub fn go(self: App<'a>) {

    }


}

