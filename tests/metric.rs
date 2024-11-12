

use monte_carlo::Metric;


#[test]
fn test_metric() {

    let metric = Metric::new(10, 10);
    let metric = metric.get_metric();
    assert!(metric.runtime.is_some(), "Expected value of runtime to set");
    let mut metric = metric.get_metric();
    assert!(metric.runtime.is_some(), "Expected value of runtime to set");

    metric.update_cache_hits(123);
    metric.update_cache_misses(456);

    assert_eq!(metric.cache_hits, 123);
    assert_eq!(metric.cache_misses, 456);

}


