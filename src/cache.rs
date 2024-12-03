
// Debugging
#![cfg_attr(debug_assertions, allow(dead_code))]


use crate::point::Point;


/*
* Our cache
*/
#[derive(Debug)]
pub struct Cache {
    pub grid_size: u64,
    data: Vec<Vec<CacheState>>,
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

        Cache {
            grid_size: grid_size,
            data: vec![vec![CacheState::Unknown; size]; size],
            hits: 0,
            misses: 0,
        }

    } // End of new()


    /*
    * Get point from the cache.
    */
    pub fn get(&self, point: Point) -> CacheState {
        self.data[point.x as usize][point.y as usize]
    }


    /*
    * Set a point in our cache.
    */
    pub fn set(&mut self, point: Point, value: CacheState) {
        self.data[point.x as usize][point.y as usize] = value;
    }


    /*
    * Return true if the point has been set, or false otherwise.
    */
    pub fn has(&mut self, point: Point) -> bool {

        match self.data[point.x as usize][point.y as usize] {
            CacheState::True | CacheState::False => {
                self.hits += 1;
                true
            },
            CacheState::Unknown => {
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

        let r_squared = self.grid_size.pow(2);

        for x in 0..self.data.len() {
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

    }

} // End of Cache


