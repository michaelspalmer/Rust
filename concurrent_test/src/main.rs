use std::thread::Thread;

fn main() {
    let guards: Vec<_> = (0..10).map(|_| {
        Thread::scoped(|| 
            {println!("Hello");
        })
    }).collect();
}