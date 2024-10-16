use memory_model::interior_mutability::cell::*;

fn main() {
    let int_count = ClickCounter::new(54);
    int_count.click();
    println!("{:?}", int_count);
    int_count.click();
    println!("{:?}", int_count);

}