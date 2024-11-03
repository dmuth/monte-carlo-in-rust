

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

}

#[test]
fn test_grid_pi() {

    let mut grid = Grid::new(10);

    grid.update_num_points_in_circle(75);
    grid.update_num_points_not_in_circle(25);

    let pi = grid.calculate_pi();
    assert_eq!(pi, Ok(3.0) );

    let grid = Grid::new(10);

    let pi = grid.calculate_pi();
    assert!(pi.is_err() );
    if let Err(e) = pi {
        assert_eq!(e, "Attempted to divide by zero!");
    }

}

