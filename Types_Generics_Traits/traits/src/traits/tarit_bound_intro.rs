struct Game;
struct Enemy;
struct Hero;

trait Loadable {
    fn init(&self);
}

impl Game {
    fn load<T: Loadable>(&self, entity: T) {
        entity.init()
    }
}

impl Loadable for Enemy {
    fn init(&self) {
        println!("Enemy loaded");
    }
}

impl Loadable for Hero {
    fn init(&self) {
        println!("Hero loaded");
    }
}

pub fn trait_bound() {
    let game = Game;
    game.load(Enemy);
    game.load(Hero);
}
