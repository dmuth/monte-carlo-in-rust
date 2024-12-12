

use log::{info};

/*
* Our structure for displaying a progress indicator as we compute random values.
*/
#[derive(Debug)]
pub struct Progress {
    total_points: u64, // How many points in total?
    last_percent: u8, // What was the last percent we had?
}

impl Progress {

    pub fn new(total_points: u64) -> Self {

        Progress {
            total_points: total_points,
            last_percent: 0,
        }

    }


    /*
    * Check to see if we should display debugging info for our progress, and return
    * true if we did, false otherwise.
    */
    pub fn display(&mut self, num_points: u64) -> bool {

        let percent = self.get_percent(num_points);

        if percent > self.last_percent {
            info!("Progress: {:?}/{:?} points ({:?}%)", num_points, self.total_points, percent);
            self.last_percent = percent;
            return true;
        }

        false

    }


    /*
    * Get our current percentage progress
    */
    pub fn get_percent(&mut self, num_points: u64) -> u8 {
        let percent = (
            (num_points as f64 / self.total_points as f64) * 100.0 ).floor() as u8;
        percent
    }


} // impl Progress



