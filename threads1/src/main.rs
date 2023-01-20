use std::sync::{Arc,RwLock};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(RwLock::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            (*status_shared.write().unwrap()).jobs_completed += 1;
        }
    });
    while (*status.read().unwrap()).jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
