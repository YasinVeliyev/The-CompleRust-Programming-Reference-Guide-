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
use std::collections::btree_map::OccupiedEntry;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
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
    let mut hash = HashMap::<String, u64>::new();
    let s = "k".to_string();
    hash.insert(s, 64);
    let k = match hash.get("k") {
        Some(value) => {
            let mut v = *value;
            v += 1;
            hash.insert("k".to_string(), v);
        }
        None => {
            hash.insert("k".to_string(), 1);
            ()
        }
    };
    println!("{:#?}", hash)
}
