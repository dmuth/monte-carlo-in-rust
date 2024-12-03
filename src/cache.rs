
// Debugging
#![cfg_attr(debug_assertions, allow(dead_code))]


use crate::point::Point;


/*
* Our cache
*/
#[derive(Debug)]
pub struct Cache {
    pub grid_size: u64,
    r_squared: u64,
    data: Vec<YIntercept>,
    hits: u64,
    misses: u64,
}


/*
* Possible states for values in our cache.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheState {
    True,
    False,
}

/*
* Y-intercept values.
*/
#[derive(Debug, Clone)]
enum YIntercept {
    Number(f64),
    Unknown,
}

/*
* Stats on our cache usage that are returned in a structure.
*/
#[derive(Debug)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
}


impl Cache {

    pub fn new(grid_size: u64) -> Self {

        //
        // Putting this here just in case this ever gets compiled on a 32-bit system.
        //
        assert!(grid_size <= usize::MAX as u64, "Cache::new(): Value out of range for usize");

        //
        // Add one because if the grid is say, 10 points, the values 
        // range from 0 to 10 inclusive.
        //
        let size = grid_size as usize + 1;
        let r_squared = grid_size.pow(2);

        Cache {
            grid_size: grid_size,
            r_squared: r_squared,
            data: vec![YIntercept::Unknown; size],
            hits: 0,
            misses: 0,
        }

    } // End of new()


    /*
    * Based on the x value and our radius, calculate the y-intercept.
    * Any y values less than or equal to that are inside the circle.
    */
    fn get_y_intercept(&self, point: Point) -> f64 {
        let retval = (self.r_squared as f64 - point.x.pow(2) as f64).sqrt();
        retval
    }


    /*
    * Return true if the point is in the circle, false otherwise.
    */
    pub fn get(&mut self, point: Point) -> bool {

        //
        // If we don't have our y intercept, generate it.
        //
        match self.data[point.x as usize] {
            YIntercept::Unknown => {
                let y_intercept = self.get_y_intercept(point);
                self.data[point.x as usize] = YIntercept::Number(y_intercept);
                self.misses += 1;
            },
            _ => {
                self.hits += 1;
                }
        }

        //
        // Now figure out what to return.
        //
        let y_intercept = &self.data[point.x as usize];
        match y_intercept {
            YIntercept::Number(y) => {
                if point.y as f64 <= *y {
                    return true;
                } 
                return false;
            }
            _ => {
                panic!("We should never get here, so there must be a y-intercept issue!");
                },
        }

    }


    /*
    * Return true if the point has been set, or false otherwise.
    */
    pub fn has(&mut self, point: Point) -> bool {

        match self.data[point.x as usize] {
            YIntercept::Number(_) => {
                self.hits += 1;
                true
            },
            YIntercept::Unknown => {
                self.misses += 1;
                false
            }
        }

    }


    /*
    * Return our stats for this cache.
    */
    pub fn get_stats(&self) -> CacheStats {
        CacheStats{ hits: self.hits, misses: self.misses }
    }


    /*
    * Pre-compute all values in our cache.
    */
    pub fn precompute(&mut self) {

// TEST
// y = sqrt(r^2 - x^2)
/*
        let r_squared = self.grid_size.pow(2);

        for x in 0..self.data.len() {

        let r_squared = self.grid_size.pow(2);
//let tmp = self.grid_size.pow(2) as f64 - x.pow(2) as f64;
let tmp = (self.grid_size.pow(2) as f64 - x.pow(2) as f64).sqrt();
//let tmp2 = tmp.sqrt();
println!("TEST: x: {:?}, y-intercept: {:?}", x, tmp);

            for y in 0..self.data[x].len() {
                //
                // Technically, I am duplicating code from the Points struct, but I feel
                // this made more sense than splitting that out into a separate function that 
                // would be called statically from this one.
                //
                let mut val = CacheState::False;
                let total = x.pow(2) as u64 + y.pow(2) as u64;
                if total <= r_squared {
                    val = CacheState::True;
                }

                self.data[x][y] = val;

            }
        }
*/

    }

} // End of Cache


