
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
    num_points: u64,
    points: Vec<Point>,
}


impl Points {

    /*
    * Function to generate a series of points and put them into the structure.
    * 
    * rng - Our random number generator.
    * size - How big is the square we're plotting points in?
    * num_points - How many points to plot in the square?
    */
    pub fn new(rng: &mut Random, size: u64, num_points: u64) -> Self {

        let mut points = Vec::<Point>::new();

        for _i in 0..num_points {
            points.push(rng.get_point(size));
        }

        Points {
            num_points: num_points,
            points: points,
        }

    }

    /*
    * Return our points for diagnostic purposes.
    */
    pub fn get_points(self: Points) -> Vec<Point> {
        self.points
    }


    /*
    * Count the number of points in a circle.
    */
    pub fn get_points_in_circle(self: Points, mode:Option<CircleMode>) -> u64 {

        let mut retval = 0;
        let turbo: bool;

        match mode {
            Some(CircleMode::Turbo) => { 
                info!("Setting turbo mode for circle analysis!");
                turbo = true; 
            }
            None => { turbo = false; }
        }

        for point in self.points {

            if turbo {
                let total = point.x + point.y;
                if total <= self.num_points {
                    retval +=1;
                } else {
                    if point.is_in_circle(self.num_points) {
                        retval += 1;
                    }
                }

            } else {

                if point.is_in_circle(self.num_points) {
                    retval += 1;
                }

            }

        }

        retval

    }

}

