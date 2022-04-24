#[allow(dead_code, unused_labels, unused_attributes)]
#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}
#[allow(dead_code, unused_labels, unused_attributes)]
enum PlayerAction {
    Move { direction: Direction, speed: u8 },
    Wait,
    Attack(Direction),
}

#[allow(dead_code, unused_labels, unused_attributes)]
enum PaymentMode {
    Debit,
    Credit,
    PayPal,
}

fn pay_by_credit(amt: u64) {
    println!("Processing credit payment of {}", amt)
}

fn pay_by_debit(amt: u64) {
    println!("Processing debit payment of {}", amt)
}

fn paypal_redirect(amt: u64) {
    println!("Redirecting to paypal for amount {}", amt)
}

impl PaymentMode {
    fn pay(&self, amount: u64) {
        match self {
            PaymentMode::Credit => pay_by_credit(amount),
            PaymentMode::Debit => pay_by_debit(amount),
            PaymentMode::PayPal => paypal_redirect(amount),
        }
    }
}

fn get_saved_payment_mode() -> PaymentMode {
    PaymentMode::Debit
}

pub fn call_enum() {
    let simulated_player_action = PlayerAction::Move {
        direction: Direction::N,
        speed: 2,
    };
    match simulated_player_action {
        PlayerAction::Wait => println!("Player wants to wait"),
        PlayerAction::Move { direction, speed } => {
            println!(
                "Player wants to move in direction {:?} with speed {}",
                direction, speed
            )
        }
        PlayerAction::Attack(direction) => {
            println!("Player wants to attack direction {:?}", direction)
        }
    }

    let payment_mode = get_saved_payment_mode();
    payment_mode.pay(512)
}
