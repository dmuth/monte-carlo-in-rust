
// Debugging
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
//
// It is unclear to me why Points shows up as dead code, even though I am instantiating it
// in main.rs as of this writing.  I'll come back to it later.
//
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::random::Random;
use crate::point::Point;
use crate::cache::{Cache, CacheStats, CacheState};

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
    cache: Option<Cache>,
}


impl Points {

    /*
    * Function to generate a series of points and put them into the structure.
    * 
    * rng - Our random number generator.
    * grid_size - How big is the square we're plotting points in?
    * num_points - How many points to plot in the square?
    */
    pub fn new(rng: &mut Random, grid_size: u64, num_points: u64, 
        cache: Option<Cache>) -> Self {

        let mut points = Vec::<Point>::new();

        for _i in 0..num_points {
            points.push(rng.get_point(grid_size));
        }

        Points {
            points: points,
            grid_size: grid_size,
            cache: cache,
        }

    }


    /*
    * This version of the constructor is used when we want to manually insert
    * pre-generated points for testing purposes.
    */
    pub fn new_with_points(grid_size: u64, points: Vec::<Point>, 
        cache: Option<Cache>) -> Self {

        Points {
            points: points,
            grid_size: grid_size,
            cache: cache,
        }

    }


    /*
    * Return our points for diagnostic purposes.
    */
    pub fn get_points(self: Points) -> Vec<Point> {
        self.points
    }


    /*
    * Check to see if a specific point is in the circle.
    */
    fn _check_point(&self, mode: &Option<CircleMode>, r_squared: u64, point: &Point) -> bool {

        match mode {
            Some(CircleMode::Turbo) => { 
                //
                // If we're using turbo, don't bother with the square roots but instead
                // just compare the squared values.
                //
                let total = point.x.pow(2) + point.y.pow(2);
                if total <= r_squared {
                    return true;
                }
            },
            None => { 
                if point.is_in_circle(self.grid_size) {
                    return true;
                }
            }
        }

        return false;

    } // End of _check_point()


    /*
    * Our core function to get the number of points inside our circle.
    */
    pub fn _get_points_in_circle(mut self: Points, mode:Option<CircleMode>) -> (u64, CacheStats) {

        let mut num_points = 0;
        let r_squared = self.grid_size.pow(2);

        match mode {
            Some(CircleMode::Turbo) => { 
                info!("Setting turbo mode for circle analysis!");
            },
            _ => {}
        }

        for point in &self.points {

            //
            // Check the cache first (if it exists), if there's a hit, we have a value and can
            // skip the rest of this loop.
            //
            match self.cache {
                Some(ref mut cache) => {
                    if cache.has(*point) {
                        match cache.get(*point) {
                            CacheState::True => {
                                num_points += 1;
                            },
                            _ => {},
                        }
                        continue;
                    }
                },
                None => {}
            }

            //
            // Check the point manually, optionally saving the result to the cache if it exists.
            //
            let in_circle = self._check_point(&mode, r_squared, point);

            if in_circle {
                num_points += 1;
                if let Some(ref mut cache) = self.cache {
                    cache.set(*point, CacheState::True);
                }
            } else {
                if let Some(ref mut cache) = self.cache {
                    cache.set(*point, CacheState::False);
                }
            }

        }

        let stats = self.get_cache_stats();

        (num_points, stats)

    } // End of _get_points_in_circle()


    /*
    * Count the number of points in a circle.
    */
    pub fn get_points_in_circle(self: Points) -> (u64, CacheStats) {
        self._get_points_in_circle(None)
    }

    pub fn get_points_in_circle_turbo(self: Points) -> (u64, CacheStats) {
        self._get_points_in_circle(Some(CircleMode::Turbo))
    }   


    /*
    * Get stats from the cache
    */
    fn get_cache_stats(self: Points) -> CacheStats {

        match &self.cache {
            Some(cache) => {
                cache.get_stats()
            },
            None => {
                CacheStats{hits: 0, misses: 0}
            }
        }

    } // End of get_cache_stats()

}

