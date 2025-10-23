fn main() {
    let sum = |a, b| a + b;
    let result = sum(5, 10);
    println!("The sum of 5 and 10 is: {}", result);

    let multiply = |x: i32, y: i32| -> i32 {
        x * y
    };
    let product = multiply(4, 6);
    println!("The product of 4 and 6 is: {}", product);
}
