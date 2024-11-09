
// Debugging
//#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use std::fmt;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use crossbeam::channel::{self, Sender, Receiver};

use crate::random::Random;
use crate::grid::Grid;
use crate::points::Points;



/*
* Our structure.
*/
pub struct App<'a> {
    rng: &'a mut Random,
    grid_size: u64, // Size of each dimension in the grid.
    num_points: u64,
    num_points_left: u64,
    num_threads: u64,
    batch_size: u64,
    turbo: bool,
}


/*
* Our custom formatted since we can't print out the random value.
*/
impl fmt::Debug for App<'_> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("App")
            .field("random", &"<Random>") // Custom format for `Random`
            .field("grid_size", &self.grid_size)
            .field("num_points", &self.num_points)
            .field("num_points_left", &self.num_points_left)
            .field("num_threads", &self.num_threads)
            .field("batch_size", &self.batch_size)
            .field("turbo", &self.turbo)
            .finish()
    }

}


impl<'a> App<'a> {

    pub fn new(rng: &'a mut Random, grid_size: u64, num_points: u64, 
        num_threads: u64, batch_size: u64, turbo: bool) -> Self {

        App {
            rng: rng,
            grid_size: grid_size,
            num_points: num_points,
            num_points_left: num_points,
            num_threads: num_threads,
            batch_size: batch_size,
            turbo: turbo,
            }

    }


    /*
    * Run things in a single thread.
    */
    fn go_single_thread(&mut self) -> f64 {

        let mut grid = Grid::new(self.grid_size);

        loop {

            let num_points = self.get_batch_count();
            if num_points == 0 {
                break
            }

            let points = Points::new(self.rng, self.grid_size, num_points);

            let points_in_circle;
            if ! self.turbo {
                points_in_circle = points.get_points_in_circle();
            } else {
                points_in_circle = points.get_points_in_circle_turbo();
            }

            let points_not_in_circle = num_points - points_in_circle;

            grid.update_num_points_in_circle(points_in_circle);
            grid.update_num_points_not_in_circle(points_not_in_circle);

        }

        let pi = grid.calculate_pi().unwrap();

        pi

    } // End of go_single_thread()


    /*
    * Spawn a single thread.
    */
    fn thread_spawn(&mut self, i:u64,
        receiver: &mut Arc< crossbeam::channel::Receiver<i32> >,
        sender: crossbeam::channel::Sender<String>
        ) -> JoinHandle<()> {

        let receiver = std::sync::Arc::clone(&receiver);
        let sender = sender.clone();

        let handle = thread::spawn(move || {
            while let Ok(task) = receiver.recv() {
                println!("Thread {} got task {}", i, task);
            }
            sender.send( format!("Hello from thread {}", i) ).expect("Error sending response!");
            println!("Thread {} is done receiving tasks", i);
        });

        handle

    } // End of thread_spawn()


    /*
    * Spawn our threads.
    */
    fn thread_spawn_all(&mut self, 
        task_receiver: crossbeam::channel::Receiver<i32>, 
        result_sender: crossbeam::channel::Sender<String>, 
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
    fn go_multi_thread(&mut self, num_threads: u64) -> f64 {

        let (task_sender, task_receiver): (Sender<i32>, Receiver<i32>) = channel::unbounded();
        let (result_sender, result_receiver): (Sender<String>, Receiver<String>) = channel::unbounded();

        let handles = self.thread_spawn_all(task_receiver, result_sender, num_threads);

        for task in 1..=10 {
            task_sender.send(task).expect("Failed to send task");
        }

        // Dropping the sender closes the channel, signaling no more tasks
        drop(task_sender);

        for result in result_receiver.iter() {
            println!("MAIN THREAD RECEIVED: {:?}", result);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        println!("DONE!");

        return 6.9;

    }


    /*
    * Our main entry point.  
    * Does all the work and returns the value of Pi.
    */
    pub fn go(mut self: App<'a>) -> f64 {

        let pi;
        if self.num_threads > 1 {
            pi = self.go_multi_thread(self.num_threads);

        } else {
            pi = self.go_single_thread();
        }
    
        pi

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

