use std::thread;

fn main() {
    let t1 = thread::spawn(|| {
        println!("I'm thread 1");
        println!("I talk about football");
        println!("I'm boring :)");
    });
    let t2 = thread::spawn(|| {
        println!("I'm thread 2");
        println!("I'm interested in tennis");
        println!("I like to tell jokes :(");
    });
    t1.join();
    t2.join();
}