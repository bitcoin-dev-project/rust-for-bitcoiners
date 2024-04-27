#![allow(unused)]
use std::rc::Rc;

struct Person {
    name: String,
    age: u8,
    // This structure could have 10 more fields
}

fn f(p: Person) {
    
}

fn g(boxed_p: Box<Person>) {
    // This function does not transfer the ownership of boxed_p to any other function
    // when this function exits the value pointed by boxed_p will be freed
    // This is why Box is a smart pointer
}
// dyn Box<Trait>

fn main() {
    // When exactly we need Box? i.e heap allocation?
    // Why you might need to explicitly allocate some value in heap?
    let some_string: String = "afenquefwinefnabfihbkaewnek".to_owned();
    // String type has the pointer to the string data in heap and information about it's length and capacity
    let boxed_string: Box<String> = Box::new(some_string);
    // The String data itself is stored in the heap,
    // Move
    let p = Person {name: "bob".to_owned(), age: 54};
    f(p); // entire structure is moved in rust semantics but under the hood it is getting copied to the frame of function f

    let p1 = Person {name: "bob".to_owned(), age: 54};
    let pg = Box::new(p1); // this is a reference to p1 allocated in heap
    g(pg);

    let mut persons: Vec<Rc<Person>> = vec![];

    for i in 1..10 {
        persons.push(
            Rc::new(Person {
                name: format!("person_{i}"),
                age: i *2,
            })
        );
    }
    let mut persons_copy: Vec<Rc<Person>> = vec![];
    for person in persons.iter() {
        persons_copy.push(person.clone());
    }
    println!("{}", persons[0].age);
    // shared ownership
}

// Why rust restricts that at any point there can be only one mutable reference?

// Deref Trait demonstration

fn deref_coersion(v: &String) {
    // Do something
}

fn derefmut_coersion(v: &mut String) {
    // Do something
}

fn deref(v: &Box<String>) {
    // Do Something
}

fn derefmut(v: &mut Box<String>) {
    // Do Something
}

fn deref_demo() {
    let ex: String = "example".to_string();
    let a: Box<String> = Box::new(ex);
    deref_coersion(&a);
    deref(&a);

    let mut ex: String = "example".to_string();
    let mut a: Box<String> = Box::new(ex);
    derefmut_coersion(&mut a);
    derefmut(&mut a);
}
