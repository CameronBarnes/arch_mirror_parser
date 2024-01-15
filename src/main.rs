use std::process::Command;

use once_cell::sync::Lazy;

mod types;
mod parsing;

static IS_WINDOWS: bool = cfg!(windows);
static HAS_RSYNC: Lazy<bool> = Lazy::new(check_for_rsync);

pub fn check_for_rsync() -> bool {
    Command::new("which").arg("rsync").output().unwrap().status.success()
}

fn main() {
    println!("Hello, world!");
}
