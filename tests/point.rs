

use monte_carlo::Point;


#[test]
fn test_get_point() {

    let point = Point{x:1, y:2};
    assert_eq!(point.x, 1);
    assert_eq!(point.y, 2);

}


