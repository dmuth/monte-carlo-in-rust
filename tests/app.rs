

use monte_carlo::App;
use monte_carlo::Random;


#[test]
fn test_app() {

    let mut rng = Random::new(Some(12345));

    let app = App::new(&mut rng, 10, 10, 1, false);
    //pub fn new(rng: &mut Random, size: u64, num_points: u64, num_threads: u64, turbo: bool) -> Self {
    println!("TEST APP: {:?}", app);

}


