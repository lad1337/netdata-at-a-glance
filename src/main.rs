use std::time::{Duration, SystemTime};
use std::thread::sleep;

pub mod config;
pub mod netdata;
pub mod pushover;

use config::Config;

fn main() {
    let c = Config::load();
    println!("{:?}", c);
    let check_timeout = Duration::new(5, 0);
    let update_timeout = Duration::new(60 * c.update_cooldown, 0);
    let mut last_update = SystemTime::now() - update_timeout;
    let mut last_value: i64 = 0;
    loop {
        let now = SystemTime::now();
        let value = netdata::get_value(&c);

        if last_value != value {
            let next_update = last_update + update_timeout;
            if now > next_update  {
                pushover::update_glance(&c, value);
                last_update = now;
                last_value = value;
            }else {
                println!(
                    "Not updating: to early, next in {}min",
                    next_update.duration_since(now)
                    .unwrap()
                    .as_secs() / 60,
                );
            }
        }else {
            println!("Not updating: same value");
        }
        sleep(check_timeout);
    }

}

