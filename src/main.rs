use std::env;
use rand::prelude::*;

struct Process {
    arrival_time: i8,
    completed_time: i8
}

struct FIFOQueue<T> {
    queue: Vec<T>
}

impl <T> FIFOQueue<T> {
    fn new() -> Self {
        FIFOQueue { queue: Vec::new() }
    }

    fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let numb_ps = 3;
    let mut rng = thread_rng();
    let mut queue: FIFOQueue<Process> = FIFOQueue::new();
    for x in 0..3 {
        let arrival_time = 0;
        let completed_time = rng.gen_range(0..10);
        let process = Process { arrival_time, completed_time };
        queue.enqueue(process);
    }

    let mut total_time = 0;
    let mut acc_time = 0;
    for x in 0..3 {
        let process = queue.dequeue();
        total_time += process.completed_time;
        println!("Completion Time: {} ticks", total_time);
        acc_time += total_time;
    }

    println!("Average {} ticks", acc_time / 3);
}
