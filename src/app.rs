
// Debugging
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use std::fmt;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use crossbeam::channel::{self, Sender, Receiver};
use log::{info, debug};

use crate::cache::CacheStats;
use crate::grid::Grid;
use crate::points::Points;
use crate::metric::Metric;
use crate::metrics::Metrics;
use crate::random::Random;


type ResultMessage = (u64, u64, Metric);

/*
* Our structure.
*/
pub struct App {
    grid_size: u64, // Size of each dimension in the grid.
    num_points: u64,
    num_points_left: u64,
    num_threads: u64,
    batch_size: u64,
    turbo: bool,
    random_seed: Option<u64>,
}


/*
* Our custom formatted since we can't print out the random value.
*/
impl fmt::Debug for App {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("App")
            .field("grid_size", &self.grid_size)
            .field("num_points", &self.num_points)
            .field("num_points_left", &self.num_points_left)
            .field("num_threads", &self.num_threads)
            .field("batch_size", &self.batch_size)
            .field("turbo", &self.turbo)
            .field("random_seed", &self.random_seed)
            .finish()
    }

}


impl App {

    pub fn new(grid_size: u64, num_points: u64, 
        num_threads: u64, batch_size: u64, turbo: bool, random_seed: Option<u64>) -> Self {

        App {
            grid_size: grid_size,
            num_points: num_points,
            num_points_left: num_points,
            num_threads: num_threads,
            batch_size: batch_size,
            turbo: turbo,
            random_seed: random_seed,
            }

    }


    /*
    * Run things in a single thread.
    */
    fn go_single_thread(&mut self) -> (f64, Metrics) {

        let mut grid = Grid::new(self.grid_size);
        let mut metrics = Metrics::new(self.grid_size);

        loop {

            let num_points = self.get_batch_count();
            if num_points == 0 {
                break
            }

            //
            // We're creating our metric here, because we want single thread
            // metrics to mirror how they're created in multi-threading as 
            // closely as possible.
            //
            let mut metric = Metric::new();
            metric.update_num_points(num_points);

            let mut rng = Random::new(self.random_seed);
            let points = Points::new(&mut rng, self.grid_size, num_points, None);

            let points_in_circle;
            let cache_stats: CacheStats;
            if ! self.turbo {
                (points_in_circle, cache_stats) = points.get_points_in_circle();
            } else {
                (points_in_circle, cache_stats) = points.get_points_in_circle_turbo();
            }

            let points_not_in_circle = num_points - points_in_circle;
            metric.update_cache_hits(cache_stats.hits);
            metric.update_cache_misses(cache_stats.misses);
            metrics.add_metric(metric);

            grid.update_num_points_in_circle(points_in_circle);
            grid.update_num_points_not_in_circle(points_not_in_circle);

        }

        let pi = grid.calculate_pi().unwrap();

        (pi, metrics)

    } // End of go_single_thread()


    /*
    * Our core thread function.
    */
    fn thread_spawn_core(thread_id: u64,
        receiver: Arc< crossbeam::channel::Receiver<u64> >,
        sender: crossbeam::channel::Sender< ResultMessage >,
        random_seed: Option<u64>, grid_size: u64, turbo: bool
        ) {

            let mut rng = Random::new(random_seed);

            while let Ok(num_points) = receiver.recv() {

                debug!("Thread {} needs to calculate {} points.", thread_id, num_points);

                let mut metric = Metric::new();
                metric.update_num_points(num_points);

                let points = Points::new(&mut rng, grid_size, num_points, None);

                let points_in_circle;
                let cache_stats: CacheStats;
                if ! turbo {
                    (points_in_circle, cache_stats) = points.get_points_in_circle();
                } else {
                    (points_in_circle, cache_stats) = points.get_points_in_circle();
                }

                metric.update_cache_hits(cache_stats.hits);
                metric.update_cache_misses(cache_stats.misses);

                let points_not_in_circle = num_points - points_in_circle;
                sender.send( (points_in_circle, points_not_in_circle, metric) 
                    ).expect("Error sending response!");

            }

    } // End of thread_spawn_core()


    /*
    * Spawn a single thread.
    */
    fn thread_spawn(&mut self, thread_id:u64,
        receiver: &mut Arc< crossbeam::channel::Receiver<u64> >,
        sender: crossbeam::channel::Sender< ResultMessage >
        ) -> JoinHandle<()> {

        let receiver = std::sync::Arc::clone(&receiver);
        let sender = sender.clone();
        //
        // Why are we creating those variables instead of just passing in self.turbo, etc.?
        // Turns out that doing that causes self to potentially leak outside of the thread,
        // and the Rust compiler doesn't like that.
        //
        let random_seed = self.random_seed;
        let grid_size = self.grid_size;
        let turbo = self.turbo;

        let handle = thread::spawn(move || {
            Self::thread_spawn_core(thread_id, receiver, sender, random_seed, grid_size, turbo );
        });

        handle

    } // End of thread_spawn()


    /*
    * Spawn our threads.
    */
    fn thread_spawn_all(&mut self, 
        task_receiver: crossbeam::channel::Receiver<u64>, 
        result_sender: crossbeam::channel::Sender< ResultMessage >, 
        num_threads: u64) -> Vec< JoinHandle<()> > {

        // 
        // The receiver needs to be in Arc for thread safety, but the
        // sender does not.
        //
        let mut task_receiver = std::sync::Arc::new(task_receiver);
        let mut handles = vec![];

        for i in 0..num_threads {
            let handle = self.thread_spawn(i, &mut task_receiver, result_sender.clone());
            handles.push(handle);
        }

        handles

    } // End of thread_spawn_all()


    /*
    * Solve with multiple threads.
    */
    fn go_multi_thread(&mut self, num_threads: u64) -> (f64, Metrics) {

        let (task_sender, task_receiver): (Sender<u64>, Receiver<u64>) = channel::unbounded();
        let (result_sender, result_receiver): (
            Sender< ResultMessage >, Receiver< ResultMessage >) = channel::unbounded();

        // Spin up our threads
        let handles = self.thread_spawn_all(task_receiver, result_sender, num_threads);

        //
        // Now send off all of our messages to our threads.
        //
        loop {

            let num_points = self.get_batch_count();
            if num_points == 0 {
                break
            }

            task_sender.send(num_points).expect("Failed to send num_points");

        }

        //
        // Dropping the variable will close the channel, so all 
        // threads will exit when done.
        //
        drop(task_sender);

        let mut grid = Grid::new(self.grid_size);
        let mut metrics = Metrics::new(self.grid_size);

        //
        // Loop through our results and update our grid and metrics.
        //
        for result in result_receiver.iter() {

            debug!("MAIN THREAD RECEIVED: {:?}", result);

            let (points_in_circle, points_not_in_circle, metric) = result;
            metrics.add_metric(metric);

            grid.update_num_points_in_circle(points_in_circle);
            grid.update_num_points_not_in_circle(points_not_in_circle);

        }

        // Wait for all threads to complete
        debug!("Waiting for threads to complete...");
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        info!("Done reading results from threads!");

        let pi = grid.calculate_pi().unwrap();
        return (pi, metrics);

    }


    /*
    * Our main entry point.  
    * Does all the work and returns the value of Pi.
    */
    pub fn go(mut self: App) -> (f64, Metrics) {

        let pi;
        let metrics;
        if self.num_threads > 1 {
            (pi, metrics) = self.go_multi_thread(self.num_threads);

        } else {
            (pi, metrics) = self.go_single_thread();
        }
    
        (pi, metrics)

    } // End of go()


    /*
    * Get a number of points to create.  This returns the batch size unless
    * we have less than a single batch size remaining, at which point it returns 0.
    */
    pub fn get_batch_count(&mut self) -> u64 {

        if self.num_points_left < self.batch_size {
            let retval = self.num_points_left;
            self.num_points_left = 0;
            return retval;
        }

        self.num_points_left -= self.batch_size;
        return self.batch_size;

    }

}

