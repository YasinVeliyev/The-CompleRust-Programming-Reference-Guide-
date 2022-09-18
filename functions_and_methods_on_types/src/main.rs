mod enum_methods;
mod enums;
mod structs;
use enum_methods::*;
use enums::_enum;
use structs::_struct;

fn main() {
    _enum();
    _struct();
    let payment_mode = get_saved_payment_mode();
    payment_mode.pay(512);
}
