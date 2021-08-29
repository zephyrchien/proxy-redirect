use std::env;
use std::process;
use std::sync::Arc;

mod conf;
mod relay;
mod rules;
use conf::Config;

const USAGE: &str = concat!("usage: relay -c <config>");

fn load_conf() -> Config {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "-c" {
        eprintln!("{}", USAGE);
        process::exit(1);
    };
    Config::from_file(&args[2])
}

fn main() {
    // init logger
    env_logger::init();

    // load config
    log::warn!("load config");
    let conf = load_conf();
    eprintln!("{}", &conf);

    // build scheduler and run
    log::warn!("service start");
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(relay::run(Arc::new(conf)))
}
