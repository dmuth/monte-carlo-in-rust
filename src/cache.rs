
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
    data: Vec<YArc>,
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
* Y-arc values.
*/
#[derive(Debug, Clone)]
enum YArc {
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
            data: vec![YArc::Unknown; size],
            hits: 0,
            misses: 0,
        }

    } // End of new()


    /*
    * Based on the x value and our radius, calculate the y-arc.
    * Any y values less than or equal to that are inside the circle.
    */
    fn get_y_arc(&self, x: u64) -> f64 {
        let retval = (self.r_squared as f64 - x.pow(2) as f64).sqrt();
        retval
    }


    /*
    * Return true if the point is in the circle, false otherwise.
    *
    * NOTE: This strays a little from the original intent, which was to return
    * a cached value.  This is because the cache functionality changed a bit--it started
    * off as a 2 dimensional array of all possible points, but I quickly ran into
    * scaling issues.
    */
    pub fn get(&mut self, point: Point) -> bool {

        //
        // If we don't have our y arc in our cache, generate it.
        //
        match self.data[point.x as usize] {
            YArc::Unknown => {
                let y_arc = self.get_y_arc(point.x);
                self.data[point.x as usize] = YArc::Number(y_arc);
                self.misses += 1;
            },
            _ => {
                self.hits += 1;
                }
        }

        //
        // Now figure out what to return.
        //
        let y_arc = &self.data[point.x as usize];
        match y_arc {
            YArc::Number(y) => {
                if point.y as f64 <= *y {
                    return true;
                } 
                return false;
            }
            _ => {
                panic!("We should never get here, so there must be a y-arc issue!");
                },
        }

    }


    /*
    * Return true if the point has been set, or false otherwise.
    */
    pub fn has(&mut self, point: Point) -> bool {

        match self.data[point.x as usize] {
            YArc::Number(_) => {
                self.hits += 1;
                true
            },
            YArc::Unknown => {
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
    * Pre-compute all y-arc values in our cache.
    */
    pub fn precompute(&mut self) {

        for x in 0..self.data.len() {
            let y_arc = self.get_y_arc(x as u64);
            self.data[x] = YArc::Number(y_arc);
        }

    }

} // End of Cache


