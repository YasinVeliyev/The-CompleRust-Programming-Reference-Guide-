use std::fmt::{Display, format};
use slog::Logger;
use crate::weapon::RailGun;
use crate::PlayingCharacter;


pub struct Enemy {
    name:String,
    logger:Logger,
    weapon:RailGun
}

impl Enemy{
    pub fn new(logger:&Logger,name:&str) ->Self {
        let player_log=logger.new(o!("Enemy" => format!("{}",name)));
        let weapon_log = player_log.new(o!("RailGun" => "S12"));
        Self {
            name:name.to_string(),
            logger:player_log,
            weapon:RailGun(weapon_log)
        }
    }
}

impl PlayingCharacter for Enemy {
    fn shoot(&self) {
        warn!(self.logger,"{} shooting with {}",self.name,self.weapon);
        self.weapon.fire();
    }
}