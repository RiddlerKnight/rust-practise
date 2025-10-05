fn main() {
    // ownership with strings
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid because s2 now owns the data
    // println!("{}", s1); // this would cause a compile-time error

    println!("{}", s2); // this is valid because s2 owns the data

    println!("------------------");

    // let mut s3 = String::from("ABC");
    // do_something(s3); // s3 is moved to the function and then returned back
    // println!("{}", s3); // this would cause a compile-time error because s3 was moved to the function

    println!("------------------");

    let s4 = String::from("SFourth");
    let s5 = &s4; // s5 is a reference to s4, s4 is still valid because ownership is not transferred just borrowed
    println!("{}", s4); // this is valid because s4 still owns the data
    println!("{}", s5); // this is valid because s5 is a reference to s4

    println!("------------------");

    let mut s6 = String::from("SSixth");
    let s7 = &mut s6; // s7 is a mutable reference to s6, s6 is still valid because ownership is not transferred just borrowed
    println!("{}", s7); // this is valid because s7 is a mutable reference to s6
    s7.push_str(" is changed"); // we can change the data through the mutable reference
    println!("{}", s7); // this is valid because s7 is a mutable reference to s6
    println!("{}", s6); // this is valid because s6 still owns the data

    println!("------------------");

    let mut treasure = String::from("Gold");
    {
        let r1 = &mut treasure; // r1 is a mutable reference to treasure
        r1.push_str(" and Silver"); // we can change the data through the mutable reference
        println!("Inside inner scope: {}", r1); // this is valid because r1 is a mutable reference to treasure
    } // r1 goes out of scope here, so we can create a new mutable reference
    let r2 = &mut treasure; // r2 is a new mutable reference to treasure
    r2.push_str(" and Bronze"); // we can change the data through the mutable reference
    println!("Outside inner scope: {}", r2); // this is valid because r2 is a mutable reference to treasure

    println!("------------------");

    let mut treasure2 = String::from("Diamond");
    let borrowed_treasure2 = &mut treasure2; // borrowed_treasure2 is a mutable reference to treasure2
    let borrowed2_treasure2 = &mut treasure2; // error: cannot borrow `treasure2` as mutable more than once at a time
    println!("Mutable references: {} and {}", borrowed_treasure2, borrowed2_treasure2);
}

fn do_something(s: String) -> String {
    s
}
