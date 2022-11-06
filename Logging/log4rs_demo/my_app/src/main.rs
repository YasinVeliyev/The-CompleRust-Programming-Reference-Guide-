
use log::error;
use my_lib::Config;
fn main() {
    Config::load_global_config();

    match log4rs::init_file("config/log.yaml",Default::default()){
        Ok(_) => {}
        Err(err) => {eprintln!("{}",err)}
    };
    error!("Sample app v{}",env!("CARGO_PKG_VERSION"));
    Config::load_global_config();
}
