struct Player {
    name: String,
    health: u8
}

impl Player {
    fn new(name: String, health: u8) -> Self {
        Player { name, health }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_health(&self) -> u8 {
        self.health
    }

    fn take_damage(&mut self, damage: u8) {
        if damage >= self.health {
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }

    fn healing(&mut self, heal: u8) {
        self.health += heal;
        if self.health > 100 {
            self.health = 100; // Assuming max health is 100
        }
    }
}

fn main() {
    let mut player1 = Player::new(String::from("Alice"), 100);
    let mut player2 = Player::new(String::from("Bob"), 80);

    player1.take_damage(30);
    player2.healing(10);

    println!("{} has {} health left.", player1.get_name(), player1.get_health());
    println!("{} has {} health left.", player2.get_name(), player2.get_health());

    player1.take_damage(80);
    player2.take_damage(50);
    println!("{} has {} health left.", player1.get_name(), player1.get_health());
    println!("{} has {} health left.", player2.get_name(), player2.get_health());

    player1.healing(10);
    player2.healing(30);
    println!("{} has {} health left.", player1.get_name(), player1.get_health());
    println!("{} has {} health left.", player2.get_name(), player2.get_health());

}
