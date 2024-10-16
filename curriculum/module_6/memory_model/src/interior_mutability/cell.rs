use std::cell::Cell;

#[derive(Debug)]
pub struct ClickCounter<T> {
    pub value: T,
    count: Cell<u32>,
}

impl<T> ClickCounter<T> {
    pub fn new(value: T) -> ClickCounter<T> {
        ClickCounter {value, count: Cell::new(0)}
    }

    pub fn click(&self) {
        self.count.set(
            self.count.get() + 1
        )
    }
}
