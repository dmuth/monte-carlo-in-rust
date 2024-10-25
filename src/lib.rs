

//
// Declare our random module here so that unit tests can access it, since they
// are in another crate.
//
mod random;
pub use random::Random;

//
// Declare the points module here so that other files in this crate can use it.
//
mod points;
pub use points::Point;

