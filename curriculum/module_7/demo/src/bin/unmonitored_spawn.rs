use std::thread;
use std::time::Duration;

fn main() {
    // New Thread invoked from this main thread
    thread::spawn(|| {
        println!("Hi I'm different from main!");
        println!("You may not hear my full message...");
        println!("main might die sooner than me...");
        println!("I have to die with main :(");
        for i in 1..1000 {
            println!("{i}");
        }
    });

    // main thread sleeps
    thread::sleep(Duration::from_nanos(1000000));
    println!("Exiting main");
}
