

use monte_carlo::Point;


#[test]
fn test_get_point() {

    let point = Point{x:1, y:2};
    assert_eq!(point.x, 1);
    assert_eq!(point.y, 2);

}

#[test]
fn test_is_point_in_circle() {

    let grid_size = 10;

    let point = Point{x:1, y:2};
    assert_eq!(point.is_in_circle(grid_size), true);

    let point = Point{x:5, y:5};
    assert_eq!(point.is_in_circle(grid_size), true);

    let point = Point{x:10, y:10};
    assert_eq!(point.is_in_circle(grid_size), false);

}

