mod worker;
use worker::Worker;

fn main() {
    let mut workers: Vec<Worker> = Vec::new();
    for i in 0..=9 {
        workers.push(Worker::new(i));
    }
    for worker in workers {
        worker.join();
    }
}