
// Debugging
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]


use std::time::Duration;

use serde::Serialize;
use serde::Serializer;


/*
* Our metric.
*/
#[derive(Debug, Serialize)]
pub struct Metrics {
    #[serde(serialize_with = "serialize_duration")]
    pub runtime: Option<Duration>,
    grid_size: u64,
    pub num_points: u64,
    pub num_metrics: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}


/*
* Function to serialize our duration, by turning it from a Duration into a string.
*/
fn serialize_duration<S>(duration: &Option<Duration>, serializer: S
    ) -> Result<S::Ok, S::Error> where S: Serializer {

    match duration {

        Some(duration) => {
            let secs = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
            serializer.serialize_f64(secs)
        }

        None => serializer.serialize_none(), // Serialize as null

    }

}


impl Metrics {

    pub fn new(grid_size: u64) -> Self {

        let metrics = Metrics {
            runtime: None,
            grid_size: grid_size, 
            num_points: 0,
            num_metrics: 0,
            cache_hits: 0,
            cache_misses: 0,
            };

        metrics

    }

}


