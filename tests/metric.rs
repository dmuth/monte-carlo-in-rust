

use monte_carlo::Metric;


#[test]
fn test_metric() {

    let metric = Metric::new();
    let metric = metric.get_metric();
    assert!(metric.runtime.is_some(), "Expected value of runtime to set");
    let mut metric = metric.get_metric();
    assert!(metric.runtime.is_some(), "Expected value of runtime to set");

    metric.update_cache_hits(123);
    assert_eq!(metric.cache_hits, 123);
    metric.update_cache_hits(123);
    assert_eq!(metric.cache_hits, 246);

    metric.update_cache_misses(456);
    assert_eq!(metric.cache_misses, 456);
    metric.update_cache_misses(456);
    assert_eq!(metric.cache_misses, 912);

    assert_eq!(metric.num_points, 0);

    metric.update_num_points(100);
    assert_eq!(metric.num_points, 100);
    
    metric.update_num_points(50);
    assert_eq!(metric.num_points, 150);


}


