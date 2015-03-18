use std::sync::{Arc, Mutex};
use std::thread;
use std::old_io::timer;
use std::time::Duration;

fn main() {

    let data = Arc::new(Mutex::new(vec![1u32, 2, 3]));
    
    for i in 0..2 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        });
    }
    
    timer::sleep(Duration::milliseconds(50));
}