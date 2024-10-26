
// Debugging
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
//
// It is unclear to me why Points shows up as dead code, even though I am instantiating it
// in main.rs as of this writing.  I'll come back to it later.
//
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::random::Random;


/*
* Our point.
*/
#[derive(Debug)]
pub struct Point {
    pub x: u64,
    pub y: u64
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

}

