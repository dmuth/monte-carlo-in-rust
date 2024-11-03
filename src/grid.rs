
// Debugging
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]


/*
* Our Grid structure, used to represent our grid.
*/
#[derive(Debug)]
pub struct Grid {
    // The size of each side, in points.
    size: u32,
    num_points_in_circle: u64,
    num_points_not_in_circle: u64,
}


impl Grid {

    pub fn new(size: u32) -> Self {
        Grid {
            size: size,
            num_points_in_circle: 0,
            num_points_not_in_circle: 0,
        }
    }

    pub fn get_num_points_in_circle(&self) -> u64 {
        self.num_points_in_circle
    }

    pub fn update_num_points_in_circle(&mut self, num: u64) {
        self.num_points_in_circle += num;
    }

    pub fn get_num_points_not_in_circle(&self) -> u64 {
        self.num_points_not_in_circle
    }

    pub fn update_num_points_not_in_circle(&mut self, num: u64) {
        self.num_points_not_in_circle += num;
    }

}



