fn main() {
    println!("Hello, world!");

    variables();
    scopes();
}

fn variables(){
    let foo = 5; // immutable by default and type inferred

    #[allow(unused_mut)]
    let mut bar: i32 = 10; // mutable variable with explicit type
    println!("foo: {}, bar: {}", foo, bar);

    bar += 5;
    println!("Updated bar: {}", bar);

    // foo += 1; // This will cause a compile-time error because foo is immutable
    println!("============================");
}

fn scopes(){
    let x = 10;
    {
        let y = 20;
        println!("x: {}, y: {}", x, y);
    }
    // println!("y: {}", y); // This will cause a compile-time error because y is out of scope
    {
        let x = 30; // shadows outer x
        println!("Inner x: {}", x);
    }
    println!("Outer x: {}", x);
    println!("============================");
}