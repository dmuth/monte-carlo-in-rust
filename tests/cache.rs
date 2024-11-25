
use std::rc::Rc;
use std::cell::RefCell;

use monte_carlo::Point;
use monte_carlo::Cache;
use monte_carlo::CacheState;


#[test]
fn test_cache_get_set_has() {

    let grid_size = 5;
    let cache = Rc::new(RefCell::new(Cache::new(grid_size)));

    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (0, 0));

    let point = Point{x:1, y:2};

    assert_eq!(cache.borrow().get(point), CacheState::Unknown);
    assert_eq!(cache.borrow_mut().has(point), false);

    cache.borrow_mut().set(point, CacheState::True);
    assert_eq!(cache.borrow().get(point), CacheState::True);
    assert_eq!(cache.borrow_mut().has(point), true);

    cache.borrow_mut().set(point, CacheState::False);
    assert_eq!(cache.borrow().get(point), CacheState::False);
    assert_eq!(cache.borrow_mut().has(point), true);

    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (2, 1));

    let point = Point{x:3, y:4};
    assert_eq!(cache.borrow().get(point), CacheState::Unknown);
    assert_eq!(cache.borrow_mut().has(point), false);

    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (2, 2));

}


