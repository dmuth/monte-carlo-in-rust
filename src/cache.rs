
// Debugging
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]


use crate::Point;


/*
* Our cache
*/
#[derive(Debug, Clone)]
pub struct Cache {
    pub grid_size: u64,
    data: Vec<Vec<CacheState>>,
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


impl Cache {

    pub fn new(grid_size: u64) -> Self {

        //
        // Putting this here just in case this ever gets compiled on a 32-bit system.
        //
        assert!(grid_size <= usize::MAX as u64, "Cache::new(): Value out of range for usize");

        let size = grid_size as usize;

        Cache {
            grid_size: grid_size,
            data: vec![vec![CacheState::Unknown; size]; size],
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
    pub fn has(&self, point: Point) -> bool {

        match self.data[point.x as usize][point.y as usize] {
            CacheState::True | CacheState::False => {
                true
            },
            CacheState::Unknown => {
                false
            }
        }

    }


} // End of Cache



