
// Debugging
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]


/*
* Our point.
*/
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: u64,
    pub y: u64
}

impl Point {

    /*
    * Return our points for diagnostic purposes.
    */
    pub fn is_in_circle(self: Point, grid_size: u64) -> bool {
        let hyp = ( 
            ( self.x.pow(2) + self.y.pow(2) ) 
            as f64).sqrt();

        if hyp <= grid_size as f64 {
            return true
        }

        return false

    }

}
