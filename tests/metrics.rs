

use monte_carlo::Metric;
use monte_carlo::Metrics;


#[test]
fn test_metrics() {

    let mut metric = Metric::new();
    metric.update_cache_hits(123);
    metric.update_cache_misses(456);
    let mut metric2 = Metric::new();
    metric2.update_cache_hits(123);
    metric2.update_cache_misses(456);
    let metric3 = Metric::new();

    let mut metrics = Metrics::new(10);
    metrics.add_metric(metric);
    metrics.add_metric(metric2);
    metrics.add_metric(metric3);

    assert!(metrics.runtime.is_some(), "Expected value of runtime to set");
    assert_eq!(metrics.num_metrics, 3);
    assert_eq!(metrics.cache_hits, 246);
    assert_eq!(metrics.cache_misses, 912);

}


