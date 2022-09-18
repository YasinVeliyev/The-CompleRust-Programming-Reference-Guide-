pub enum PaymentMode {
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

fn pay_by_paypal(amt: u64) {
    println!("Redirecting to paypal payment of {}", amt)
}

impl PaymentMode {
    pub fn pay(&self, amount: u64) {
        match self {
            PaymentMode::Debit => pay_by_debit(amount),
            PaymentMode::Credit => pay_by_credit(amount),
            PaymentMode::PayPal => pay_by_paypal(amount),
        }
    }
}

pub fn get_saved_payment_mode() -> PaymentMode {
    PaymentMode::Debit
}
