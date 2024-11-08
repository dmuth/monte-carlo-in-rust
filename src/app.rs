
// Debugging
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use std::fmt;

use crate::random::Random;
use crate::grid::Grid;
use crate::points::Points;


/*
* Our structure.
*/
pub struct App<'a> {
    rng: &'a mut Random,
    size: u64, // Size of each dimension in the grid.
    num_points: u64,
    num_points_left: u64,
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
            .field("num_points_left", &self.num_points_left)
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
            num_points_left: num_points,
            num_threads: num_threads,
            batch_size: batch_size,
            turbo: turbo,
            }

    }


    /*
    * Our main entry point.  
    * Does all the work and returns the value of Pi.
    */
    pub fn go(mut self: App<'a>) -> f64 {

        if self.num_threads > 1 {
            panic!("Thead count > 1 currently not supported!");
        }
     
        let mut grid = Grid::new(self.size);


        loop {

            let num_points = self.get_batch_count();
            if num_points == 0 {
                break
            }

            let points = Points::new(self.rng, self.size, num_points);

            let points_in_circle;
            if ! self.turbo {
                points_in_circle = points.get_points_in_circle();
            } else {
                points_in_circle = points.get_points_in_circle_turbo();
            }

            let points_not_in_circle = num_points - points_in_circle;

            grid.update_num_points_in_circle(points_in_circle);
            grid.update_num_points_not_in_circle(points_not_in_circle);

        }

        let pi = grid.calculate_pi().unwrap();

        pi

    } // End of go()


    /*
    * Get a number of points to create.  This returns the batch size unless
    * we have less than a single batch size remaining, at which point it returns 0.
    */
    pub fn get_batch_count(&mut self) -> u64 {

        if self.num_points_left < self.batch_size {
            let retval = self.num_points_left;
            self.num_points_left = 0;
            return retval;
        }

        self.num_points_left -= self.batch_size;
        return self.batch_size;

    }

}

