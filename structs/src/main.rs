#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

struct Player {
    name: String,
    iq: u8,
    friends: u8,
}

impl Player {
    fn with_name(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            iq: 100,
            friends: 100,
        }
    }

    fn get_friends(&self) -> u8 {
        self.friends
    }

    fn set_friends(&mut self, count: u8) {
        self.friends = count
    }
}

enum PlayerAction {
    Move { direction: Direction, speed: u8 },
    Wait,
    Attack(Direction),
}

fn main() {
    let simulated_player_action = PlayerAction::Move {
        direction: Direction::E,
        speed: 2,
    };

    match simulated_player_action {
        PlayerAction::Attack(direction) => {
            println!("Player wants to attack direction {:?}", direction)
        }
        PlayerAction::Move { direction, speed } => {
            println!(
                "Player wants to move in direction {:?} with speed {}",
                direction, speed
            )
        }
        PlayerAction::Wait => println!("Player wants to wait"),
    }

    let mut player = Player::with_name("Yasin");
    player.set_friends(23);
    println!("{}' s friends count {}", player.name, player.get_friends());
    let friends_count = Player::get_friends(&player);
    println!("{}' s friends count {}", player.name, friends_count);
}
