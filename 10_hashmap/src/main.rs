use std::collections::HashMap;

fn main() {
    let mut weapons = HashMap::new();

    weapons.insert("Sword", 150);
    weapons.insert("Bow", 100);
    weapons.insert("Axe", 120);

    println!("--- Weapons and their damage ---");
    println!("Sword damage: {}", weapons.get("Sword").unwrap());
    println!("Bow damage: {}", weapons.get("Bow").unwrap());
    println!("Axe damage: {}", weapons.get("Axe").unwrap());
}
