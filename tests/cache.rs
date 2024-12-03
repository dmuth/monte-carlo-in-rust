
use std::rc::Rc;
use std::cell::RefCell;

use monte_carlo::Point;
use monte_carlo::Cache;


#[test]
fn test_cache_get() {

    let grid_size = 5;
    let cache = Rc::new(RefCell::new(Cache::new(grid_size)));

    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (0, 0));

    let point = Point{x:1, y:2};

    assert_eq!(cache.borrow_mut().has(point), false);
    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (0, 1));

    assert_eq!(cache.borrow_mut().get(point), true);
    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (0, 2));

    assert_eq!(cache.borrow_mut().has(point), true);
    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (1, 2));

    assert_eq!(cache.borrow_mut().get(point), true);
    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (2, 2));

    let point = Point{x:3, y:4};
    assert_eq!(cache.borrow_mut().has(point), false);
    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (2, 3));

    assert_eq!(cache.borrow_mut().get(point), true);
    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (2, 4));

    assert_eq!(cache.borrow_mut().get(point), true);
    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (3, 4));

    let point = Point{x:5, y:5};
    assert_eq!(cache.borrow_mut().get(point), false);
    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (3, 5));

    assert_eq!(cache.borrow_mut().get(point), false);
    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (4, 5));

}


#[test]
fn test_cache_precompute() {

    let grid_size = 5;
    let cache = Rc::new(RefCell::new(Cache::new(grid_size)));
    cache.borrow_mut().precompute();

    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (0, 0));

    let point = Point{x:1, y:2};

    assert_eq!(cache.borrow_mut().get(point), true);
    assert_eq!(cache.borrow_mut().has(point), true);

    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (1, 0));

    let point = Point{x:3, y:4};
    assert_eq!(cache.borrow_mut().get(point), true);
    assert_eq!(cache.borrow_mut().has(point), true);

    let point = Point{x:grid_size, y:grid_size};
    assert_eq!(cache.borrow_mut().get(point), true);
    assert_eq!(cache.borrow_mut().has(point), true);

    let stats = cache.borrow().get_stats();
    assert_eq!( (stats.hits, stats.misses), (3, 0));

    let grid_size = 10;
    let cache = Rc::new(RefCell::new(Cache::new(grid_size)));
    cache.borrow_mut().precompute();

}


