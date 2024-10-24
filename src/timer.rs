

use std::time::{Duration, Instant, SystemTime, SystemTimeError};


/*
* Our Timer structure, used to get elapsed time.
*/
pub struct Timer {
    start_time: Instant,
    start_time_t: SystemTime,
}

impl Timer {

    pub fn new() -> Self {
        let start_time = Instant::now();
        let start_time_t = SystemTime::now();
        Timer { 
            start_time: start_time, 
            start_time_t: start_time_t 
            }
    }

    /*
    * Return elapsed time.
    */
    pub fn get_elapsed(&mut self) -> Duration {
        self.start_time.elapsed()
    }

    /*
    * Return elapsed wall clock time.
    */
    pub fn get_elapsed_time_t(&mut self) -> Result<Duration, SystemTimeError> {
        self.start_time_t.elapsed()
    }

}


