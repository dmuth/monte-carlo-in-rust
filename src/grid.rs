
// Debugging
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]


/*
* Our Grid structure, used to represent our grid.
*/
#[derive(Debug)]
pub struct Grid {
    // The size of each side, in points.
    size: u64,
    num_points_in_circle: u64,
    num_points_not_in_circle: u64,
}


impl Grid {

    pub fn new(size: u64) -> Self {
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


    /*
    * Calculate Pi based on our existing values.
    */
    pub fn calculate_pi(self) -> Result<f64, &'static str> {

        let total = self.num_points_in_circle + self.num_points_not_in_circle;

        if total == 0 {
            return Err("Attempted to divide by zero!");
        }

        let quarter = self.num_points_in_circle as f64 / total as f64;
        let retval = quarter * 4.0;
        //println!("DEBUGGING: {}, {}, {}", total, quarter, retval);
        Ok(retval)

    }

}



