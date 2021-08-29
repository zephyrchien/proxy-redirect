use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug, Default)]
pub struct Config {
    pub listen: String,
    pub socks5: String,
    pub http: String,
    pub default: String,
}

impl Config {
    pub fn from_file(f: &str) -> Config {
        let mut rd = BufReader::new(File::open(f).unwrap());
        let mut config = Config::default();
        loop {
            let mut line = String::new();
            if rd.read_line(&mut line).unwrap() == 0 {
                break;
            }
            let (opt, addr) = split_kv(&line);
            load_kv(opt, addr, &mut config);
        }
        config
    }
}

fn split_kv(line: &str) -> (&str, &str) {
    let mut kv = line.splitn(2, '=');
    let key = kv.next().unwrap().trim();
    let value = kv.next().unwrap().trim();
    (key, value)
}

fn load_kv(opt: &str, addr: &str, config: &mut Config) {
    let x_opt = match opt {
        "listen" => &mut config.listen,
        "socks5" => &mut config.socks5,
        "http" => &mut config.http,
        "default" | "ss" => &mut config.default,
        _ => panic!("invalid option: {}", opt),
    };
    *x_opt = addr.into();
}
