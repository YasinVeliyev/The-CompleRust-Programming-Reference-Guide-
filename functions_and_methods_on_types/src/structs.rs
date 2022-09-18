pub struct Player {
    name: String,
    iq: u8,
    friends: u8,
}

impl Player {
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            iq: 100,
            friends: 100,
        }
    }

    pub fn get_friends(&self) -> u8 {
        self.friends
    }

    pub fn set_friends(&mut self, count: u8) {
        self.friends = count
    }
}

pub fn _struct() {
    let mut player = Player::with_name("Yasin");
    player.set_friends(23);
    println!("{}' s friends count {}", player.name, player.get_friends());
    let friends_count = Player::get_friends(&player);
    println!("{}' s friends count {}", player.name, friends_count);
}
