use std::cell::RefCell;

fn main() {
    let rf = RefCell::new(vec![]);

    let mut mut_borrow = rf.borrow_mut();
    mut_borrow.push(5);

    println!("{}", rf.borrow()[0]);

    mut_borrow.push(32);
}