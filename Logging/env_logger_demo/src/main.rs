use log::debug;
use user_auth::User;
fn main() {
    env_logger::init();
    debug!("env logger demo started");
    let user = User::new("bob", "super_sekret");
    user.sign_in("super_sekret");
    user.sign_in("super_secret");
}
