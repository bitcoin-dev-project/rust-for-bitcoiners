use std::cell::RefCell;

#[derive(Debug)]
pub struct EventRecorder<T> {
    pub value: T,
    events: RefCell<Vec<String>>,
}

impl<T> EventRecorder<T> {
    pub fn new(value: T) -> EventRecorder<T> {
        EventRecorder {value, events: RefCell::new(vec![])}
    }

    pub fn record_event(&self, event: String) {
        self.events.borrow_mut().push(event);
    }

    pub fn print_event(&self, index: usize) {
        println!("Event at {index} is {}", self.events.borrow()[index])
    }
}