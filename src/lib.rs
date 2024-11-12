

//
// Declare our random module here so that unit tests can access it, since they
// are in another crate.
//
mod random;
pub use random::Random;

//
// Declare the points modules here so that other files in this crate can use them.
//
mod point;
pub use point::Point;
mod points;
pub use points::Points;
pub use points::CircleMode;

//
// Declare our Grid module
//
mod grid;
pub use grid::Grid;

mod app;
pub use app::App;

mod timer;

mod metric;
pub use metric::Metric;

mod metrics;
pub use metrics::Metrics;


