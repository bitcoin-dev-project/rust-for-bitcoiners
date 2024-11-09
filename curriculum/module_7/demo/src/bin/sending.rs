use std::{rc::Rc, sync::Arc, thread};

#[derive(Debug)]
struct Boxed {
    f1: Box<String>
}

#[derive(Debug)]
struct Rced {
    f1: Rc<String>
}

#[derive(Debug, Clone)]
struct Arced {
    f1: Arc<String>
}
fn main() {
    // Almost all values which have a single owners are Send
    // Sending copyable values
    let int = 56;

    let _t1 = thread::spawn(move || {
        println!("{int}");
    });
    let _t2 = thread::spawn(move || {
        println!("{int}");
    });


    // Sending non copyable values
    let b_int = Box::new(32);
    thread::spawn(move || {
        println!("{b_int}");
    });

    let b = Boxed {f1: Box::new("dewaer".to_string())};
    thread::spawn(move || {
        println!("{:?}", b);
    });
    // let b = Rced {f1: Rc::new("dewaer".to_string())};
    // thread::spawn(move || {
    //     println!("{:?}", b);
    // });

    let ar = Arced {f1: Arc::new("efaw".to_string())};
    let ar1 = ar.clone();
    thread::spawn(move || {
        println!("{:?}", ar1);
    });
    println!("{:?}", ar);
}