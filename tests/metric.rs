

use monte_carlo::Metric;


#[test]
fn test_metric() {

    let metric = Metric::new(10, 10);
    let metric = metric.get_metric();

    assert!(metric.runtime.is_some(), "Expected value of runtime to set");

}


