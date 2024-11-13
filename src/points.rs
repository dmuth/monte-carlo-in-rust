
// Debugging
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
//
// It is unclear to me why Points shows up as dead code, even though I am instantiating it
// in main.rs as of this writing.  I'll come back to it later.
//
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::random::Random;
use crate::point::Point;

use log::{info};


pub enum CircleMode {
    Turbo
}


/*
* A collection of points
*/
#[derive(Debug)]
pub struct Points {
    points: Vec<Point>,
    grid_size: u64,
}


impl Points {

    /*
    * Function to generate a series of points and put them into the structure.
    * 
    * rng - Our random number generator.
    * grid_size - How big is the square we're plotting points in?
    * num_points - How many points to plot in the square?
    */
    pub fn new(rng: &mut Random, grid_size: u64, num_points: u64) -> Self {

        let mut points = Vec::<Point>::new();

        for _i in 0..num_points {
            points.push(rng.get_point(grid_size));
        }

        Points {
            points: points,
            grid_size: grid_size,
        }

    }


    /*
    * This version of the constructor is used when we want to manually insert
    * pre-generated points for testing purposes.
    */
    pub fn new_with_points(grid_size: u64, points: Vec::<Point>) -> Self {

        Points {
            points: points,
            grid_size: grid_size,
        }

    }


    /*
    * Return our points for diagnostic purposes.
    */
    pub fn get_points(self: Points) -> Vec<Point> {
        self.points
    }


    /*
    * Our core function to get the number of points inside our circle.
    */
    pub fn _get_points_in_circle(self: Points, mode:Option<CircleMode>) -> u64 {

        let mut retval = 0;
        let turbo: bool;
        let r_squared = self.grid_size.pow(2);

        match mode {
            Some(CircleMode::Turbo) => { 
                info!("Setting turbo mode for circle analysis!");
                turbo = true; 
            }
            None => { 
                turbo = false; 
            }
        }

        for point in self.points {

            if turbo {
                //
                // If we're using turbo, don't bother with the square roots but instead
                // just compare the squared values.
                //
                let total = point.x.pow(2) + point.y.pow(2);
                if total <= r_squared {
                    retval += 1;
                }

            } else {

                if point.is_in_circle(self.grid_size) {
                    retval += 1;
                }

            }

        }

        retval

    } // End of _get_points_in_circle()


    /*
    * Count the number of points in a circle.
    */
    pub fn get_points_in_circle(self: Points) -> u64 {
        self._get_points_in_circle(None)
    }

    pub fn get_points_in_circle_turbo(self: Points) -> u64 {
        self._get_points_in_circle(Some(CircleMode::Turbo))
    }   


}

