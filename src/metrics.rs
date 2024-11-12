
// Debugging
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]


use std::time::Duration;

use crate::metric::Metric;


/*
* Our metric.
*/
#[derive(Debug)]
pub struct Metrics {
    pub runtime: Option<Duration>,
    pub _grid_size: u64,
    pub _num_points: u64,
    pub num_metrics: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}


impl Metrics {

    pub fn new(grid_size: u64) -> Self {

        let metrics = Metrics {
            runtime: None,
            _grid_size: grid_size, 
            _num_points: 0,
            num_metrics: 0,
            cache_hits: 0,
            cache_misses: 0,
            };

        metrics

    }


    /*
    * Add in the values from our metrics.
    */
    pub fn add_metric(&mut self, metric: Metric) -> &mut Metrics {

        let metric = metric.get_metric();

        self.num_metrics += 1;
        self.cache_hits += metric.cache_hits;
        self.cache_misses += metric.cache_misses;
        self.runtime = Some(self.runtime.unwrap_or_default() + metric.runtime.unwrap());

        self

    } // End of add_metric()


}


