use std::sync::Arc;
use std::thread;

pub fn demo() {
    let data = Arc::new("shared data");

    for _ in 1..10 {
        let data_clone = data.clone();
        thread::spawn(move || {
            println!("I can see {}", data);
        });
    }
}