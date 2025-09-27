fn main() {
    // ownership with strings
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    // println!("{}", s1); // this would cause a compile-time error

    let mut s3 = String::from("ABC");
    do_something(s3); // s3 is moved to the function and then returned back
    println!("{}", s3); // this would cause a compile-time error because s3 was moved to the function
}

fn do_something(s: String) -> String {
    s
}
