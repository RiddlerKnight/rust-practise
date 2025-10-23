fn main() {
    let mut weapons: Vec<&str> = vec!["Sword", "Bow", "Axe", "Spear"];

    for w in weapons.iter() {
        println!("Weapon: {}", w);
    }
    println!("---");
    weapons.iter().for_each(|w| println!("Weapon via for_each: {}", w));

    println!("--- Enumerated Weapons: ---");
    weapons.iter().enumerate().for_each(|(i, w)| {
        println!("Weapon {}: {}", i + 1, w);
    });

    weapons.pop();
    println!("--- After popping the last weapon ---");
    weapons.iter().for_each(|w| println!("Weapon: {}", w));

    // Capacity and Length of the Vector
    println!("Capacity: {}", weapons.capacity());
    println!("Length: {}", weapons.len());
    // note: In Rust, Capacity will double when more space is needed.
    // note: Length is the current number of elements in the vector.

    let money = vec![100, 200, 300, 400, 500];
    let result: Vec<i32> = money.iter().map(|m| m * 2).collect();
    println!("--- Money after doubling ---");
    result.iter().for_each(|m| println!("Money: {}", m));
}
