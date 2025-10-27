//Smart pointer practice
// Box, Rc, RefCell
// Box<T> : for heap allocation, prevent recursive type size issue.
// Rc<T> : reference counting smart pointer, multiple ownership
// RefCell<T> : for interior mutability, allows mutability even when the data is immutable
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let chest: Box<i32> = Box::new(10);

    let shared_chest: Rc<RefCell<i32>> = Rc::new(RefCell::new(chest));

    **shared_chest.borrow_mut() += 5;
    **shared_chest.borrow_mut() += 10;

    println!("Chest value: {}", shared_chest.borrow());
}
