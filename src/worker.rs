use std::thread;

pub struct Worker {
    index: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(index: usize) -> Self {
        Self {
            index: index,
            handle: thread::spawn(move || {
                println!("Thread {} started.", &index);

                for i in 0..=9 {
                    println!("{}.{}", &index, i);
                }

                println!("Thread {} finished.", &index);
            }),
        }
    }

    pub fn join(self) {
        self.handle.join().unwrap();
    }
}
