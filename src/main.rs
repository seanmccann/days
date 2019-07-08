extern crate chrono;
extern crate time;

use chrono::{Utc};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut n = 0;

    if args.len() > 1 {
        n = args[1].parse::<i64>().unwrap();
    };

    if n > 0 {
        let d = Utc::now() + time::Duration::days(n);
        println!("{}", d.format("%a %b %e, %Y"));
    } else {
        println!("Can only look forward to the future.")
    }
}
