fn main() {
    // ownership with strings
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid because s2 now owns the data
    // println!("{}", s1); // this would cause a compile-time error

    // let mut s3 = String::from("ABC");
    // do_something(s3); // s3 is moved to the function and then returned back
    // println!("{}", s3); // this would cause a compile-time error because s3 was moved to the function

    let s4 = String::from("XYZ");
    let s5 = &s4; // s5 is a reference to s4, s4 is still valid because ownership is not transferred just borrowed
    println!("{}", s4); // this is valid because s4 still owns the data
    println!("{}", s5); // this is valid because s5 is a reference to s4
}

fn do_something(s: String) -> String {
    s
}
