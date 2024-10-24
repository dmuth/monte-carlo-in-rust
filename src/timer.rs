
use std::time::Duration;
use std::time::Instant;

/*
* Our Timer structure, used to get elapsed time.
*/
pub struct Timer {
    start_time: Instant,
}

impl Timer {

    pub fn new() -> Self {
        let start_time = Instant::now();
        Timer { start_time }
    }

    /*
    * Return elapsed time.
    */
    pub fn get_elapsed(&mut self) -> Duration {
        self.start_time.elapsed()
    }

}


