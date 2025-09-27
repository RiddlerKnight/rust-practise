fn main() {
    println!("Hello, world!");

    variables();
    scopes();
    memory_safety();

    println!("Function Example 1: {}", function_example_1(10));
    function_example_2(10);
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

fn memory_safety(){
    let enigmar: i32;
    if true { // eventhough this condition is always true, the compiler cannot guarantee it
        enigmar = 42; // must be initialized before use
    }else { // you must cover all possible paths to ensure initialization
        enigmar = 0;
    }
    println!("Enigmar: {}", enigmar); // This is safe because enigmar is guaranteed to be initialized
    println!("============================");
}

fn function_example_1(param: i32) -> i32 {
    param * 2 // implicit return without semicolon
    // println!("This line is unreachable"); // This will cause a compile-time warning
}

fn function_example_2(param: i32) -> i32 {
    let result = param * 2;
    println!("Result inside function: {}", result);
    return result; // explicit return
}
