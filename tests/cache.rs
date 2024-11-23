

use monte_carlo::Point;
use monte_carlo::Cache;
use monte_carlo::CacheState;


#[test]
fn test_cache_get_set_has() {

    let grid_size = 5;
    let mut cache = Cache::new(grid_size);

    let (hits, misses) = cache.get_metrics();
    assert_eq!( (hits, misses), (0, 0));

    let point = Point{x:1, y:2};

    assert_eq!(cache.get(point), CacheState::Unknown);
    assert_eq!(cache.has(point), false);

    cache.set(point, CacheState::True);
    assert_eq!(cache.get(point), CacheState::True);
    assert_eq!(cache.has(point), true);

    cache.set(point, CacheState::False);
    assert_eq!(cache.get(point), CacheState::False);
    assert_eq!(cache.has(point), true);

    let (hits, misses) = cache.get_metrics();
    assert_eq!( (hits, misses), (2, 1));

    let point = Point{x:3, y:4};
    assert_eq!(cache.get(point), CacheState::Unknown);
    assert_eq!(cache.has(point), false);

    let (hits, misses) = cache.get_metrics();
    assert_eq!( (hits, misses), (2, 2));

}


