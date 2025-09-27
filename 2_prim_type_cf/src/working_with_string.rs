pub fn run(){
    // &str is immutable string slice
    // String is growable, heap-allocated data structure

    let hello = String::from("Hello World");
    println!("Length: {}", hello.len());
    println!("Is Empty: {}", hello.is_empty());
    println!("Contains 'World': {}", hello.contains("World"));
    println!("Replace: {}", hello.replace("World", "There"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::from("Hello");
    s.push(',');
    s.push_str(" World!");
    println!("{}", s);

    let s2 = String::from("foo");
    let s3 = s2.clone();
    println!("s2: {}, s3: {}", s2, s3);

    let s4 = String::from("lo");
    let s5 = format!("He{}{}", s4, "llo");
    println!("s5: {}", s5);

    // Indexing
    let h = &s5[0..1];
    let e = &s5[1..2];
    let l = &s5[2..4];
    println!("h: {}, e: {}, l: {}", h, e, l);

    // another way to access string indexing
    println!("s5[1]: {}", s5.chars().nth(1).unwrap());
}
