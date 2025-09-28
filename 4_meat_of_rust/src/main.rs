// Structs are custom data types that let you package together related values.
// They are similar to tuples, but each field in a struct has a name, making it easier to
// understand what each value represents.
struct Point {
    x: i32,
    y: i32,
}

// Implementing methods for the Point struct
// Similar to classes in other languages, Rust allows you to define methods on structs using the `impl` keyword.
impl Point {
    // Associated function to create a new Point
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    fn get_x(&self) -> i32 {
        self.x
    }
    fn get_y(&self) -> i32 {
        self.y
    }
}

// Traits are a way to define shared behavior in Rust. They are similar to interfaces in other languages.
// A trait defines a set of methods that a type must implement to satisfy the trait.
// Here is an example of a simple trait definition.
trait Describable {
    fn describe(&self) -> String;
}

// Implementing the Describable trait for the Point struct
impl Describable for Point {
    fn describe(&self) -> String {
        format!("Point at ({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 10, y: 20 }; // Creating an instance of the Point struct
    println!("Point coordinates: ({}, {})", p.x, p.y);

    let p2 = Point::new(30, 40); // Using the associated function to create a new Point
    println!("Point coordinates: ({}, {})", p2.x, p2.y);

    println!("X coordinate: {}", p2.get_x()); // Calling a method on the Point instance
    println!("Y coordinate: {}", p2.get_y()); // Calling a method on the Point instance

    println!("Description: {}", p2.describe()); // Calling the describe method from the Describable trait
}
