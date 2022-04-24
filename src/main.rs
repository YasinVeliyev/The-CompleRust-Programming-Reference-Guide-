use project::*;
mod loops;
use loops::*;
mod structs;
use crate::structs::*;
mod closure;
use closure::*;
mod enums;
use enums::*;
mod word_counter;
use std::env;
use word_counter::call_word_counter;

fn main() {
    let arguments = env::args().collect::<Vec<String>>();

    call_closure();
    func_lib(b);
    call_loop();
    call_structs();
    call_enum();
    call_word_counter(&arguments[1]);
}
