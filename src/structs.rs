pub struct Color(pub u8, pub u8, pub u8);
pub struct Player {
    pub name: String,
    pub iq: u8,
    pub friends: u8,
    pub score: u16,
}

pub fn bump_player_socre(mut player: Player, score: u16) {
    player.score += score;
    println!();
    println!("Updated player stats:");
    println!("Name :{}", player.name);
    println!("IQ :{}", player.iq);
    println!("Friends :{}", player.friends);
    println!("Score :{}", player.score);
}

impl Player {
    fn with_name(name: &str) -> Self {
        Player {
            name: name.to_string(),
            iq: 100,
            friends: 100,
            score: 1200,
        }
    }
    fn get_friends(&self) -> u8 {
        self.friends
    }

    fn set_friends(&mut self, count: u8) {
        self.friends += count
    }
}

pub fn call_structs() {
    let _color = Color(255, 255, 255);
    let name = "Alice".to_string();
    let player = Player {
        name,
        iq: 171,
        friends: 134,
        score: 1129,
    };
    bump_player_socre(player, 120);
    let mut player1 = Player::with_name("David");
    player1.set_friends(150);
    println!(
        "{}'s friends count :{}",
        player1.name,
        player1.get_friends()
    )
}
