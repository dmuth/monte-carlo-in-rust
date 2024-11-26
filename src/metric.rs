//
// THIS FILE IS DEPRECATED.
//
// While it seemed like a good idea at the time, having some values be 
// updated per loop and others updated once per function just made abstracting
// more effort than it was worth.
//

// Debugging
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]


use std::time::Duration;

use crate::timer::Timer;


/*
* Our metric.
*/
#[derive(Debug)]
pub struct Metric {
    timer: Timer,
    pub runtime: Option<Duration>,
    pub num_points: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}


impl Metric {

    pub fn new() -> Self {

        let timer = Timer::new();

        let metric = Metric{
            timer: timer, 
            runtime: None,
            num_points: 0,
            cache_hits: 0,
            cache_misses: 0,
            };

        metric

    }


    /*
    * Update our runtime and return the metric.
    */
    pub fn get_metric(mut self: Metric) -> Metric {


        self.runtime = Some(self.timer.get_elapsed());

        self

    }


    /*
    * Update the number of points in this metric.
    */
    pub fn update_num_points(&mut self, num: u64) {
        self.num_points += num;
    }

    /*
    * Update our cache hits.
    */
    pub fn update_cache_hits(&mut self, num: u64) {
        self.cache_hits += num;
    }


    /*
    * Update our cache misses.
    */
    pub fn update_cache_misses(&mut self, num: u64) {
        self.cache_misses += num;
    }

}


