#[macro_use]
extern crate slog;

mod enemy;
mod player;
mod weapon;

use rand::Rng;
use std::{thread,time::Duration};
use slog::{Drain,Logger};
use slog_async::Async;
use player::Player;
use enemy::Enemy;

pub trait PlayingCharacter {
    fn shoot(&self);
}

struct Game {
    logger:Logger,
    player:Player,
    enemy:Enemy
}

impl Game {
    fn simulate(&mut self){
        info!(self.logger,"launching game!");
        let enemy_on_player:Vec<&dyn PlayingCharacter>=vec![&self.enemy,&self.player];
        loop {
            let mut rng=rand::thread_rng();
            let a=rng.gen_range(500..=1000);
            thread::sleep(Duration::from_millis(a));
            let player=enemy_on_player[{if a%2==0{1}else{0}}];
            player.shoot();
        }

    }
}

fn main() {
    let drain=slog_json::Json::new(std::io::stdout()).add_default_keys().build().fuse();
    let async_drain=Async::new(drain).build().fuse();
    let game_info =format!("v{}",env!("CARGO_PKG_VERSION"));
    let root_log_context =o!("Super Cool Game" => game_info);
    let root_logger=Logger::root(async_drain,root_log_context);
    let mut game=Game{
        logger:root_logger.clone(),
        player:Player::new(&root_logger,"Bob"),
        enemy:Enemy::new(&root_logger,"Malice")
    };
    game.simulate()
}
