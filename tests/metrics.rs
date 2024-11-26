

use monte_carlo::Metrics;
use monte_carlo::Timer;


#[test]
fn test_metrics() {

    let mut metrics = Metrics::new(10);
    metrics.num_points = 1000;
    metrics.num_metrics = 3;
    metrics.cache_hits = 123;
    metrics.cache_misses = 456;

    let mut timer = Timer::new();
    assert_eq!(metrics.num_points, 1000);
    assert_eq!(metrics.num_metrics, 3);
    assert_eq!(metrics.cache_hits, 123);
    assert_eq!(metrics.cache_misses, 456);

    metrics.runtime = Some(timer.get_elapsed());
    metrics.runtime = Some( timer.get_elapsed() + metrics.runtime.unwrap() );
    assert!(metrics.runtime.is_some(), "Expected value of runtime to set");

    metrics.num_metrics += 1;
    metrics.cache_hits += 123;
    metrics.cache_misses += 456;
    assert_eq!(metrics.num_points, 1000);
    assert_eq!(metrics.num_metrics, 4);
    assert_eq!(metrics.cache_hits, 246);
    assert_eq!(metrics.cache_misses, 912);

}


