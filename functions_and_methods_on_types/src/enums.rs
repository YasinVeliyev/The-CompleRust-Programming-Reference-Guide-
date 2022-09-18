#[derive(Debug)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

pub enum PlayerAction {
    Move { direction: Direction, speed: u8 },
    Wait,
    Attack(Direction),
}

pub fn _enum() {
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
}
