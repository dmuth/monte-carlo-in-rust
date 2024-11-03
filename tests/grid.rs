

use monte_carlo::Grid;


#[test]
fn test_grid() {

    let mut grid = Grid::new(10);

    assert_eq!(grid.get_num_points_in_circle(), 0);
    assert_eq!(grid.get_num_points_not_in_circle(), 0);

    grid.update_num_points_in_circle(123);
    assert_eq!(grid.get_num_points_in_circle(), 123);
    grid.update_num_points_not_in_circle(456);
    assert_eq!(grid.get_num_points_not_in_circle(), 456);

/*

Calculate value of Pi
calculate_pi()
- get total points, get ratio of points in circle to total points, multiply by 4
*/

}


