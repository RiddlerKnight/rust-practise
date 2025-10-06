struct Inventory<T> {
    item: T
}

trait DisplayItem {
    fn display(&self);
}

impl <T> DisplayItem for Inventory<T>
where T: std::fmt::Debug {
    fn display(&self){
        println!("{:?}", self.item);
    }
}

fn main() {
    let gold: Inventory<i32> = Inventory {item: 100};
    gold.display();

    let books: Inventory<&str> = Inventory {item: "Hello, Rust"};
    books.display();
}
