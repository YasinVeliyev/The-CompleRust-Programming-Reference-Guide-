pub mod super_player;
use super_player::*;

mod trait_inheritance;
use trait_inheritance::*;

mod tarit_bound_intro;
use tarit_bound_intro::*;
mod trait_objects;
use trait_objects::*;

pub fn _traits() {
    player();
    trait_inheritance();
    trait_bound();
    _objects();
}
