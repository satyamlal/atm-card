use std::process;
use std::thread;
use std::time::Duration;

pub fn exit_command() {
    let mut i: u64 = 3;
    while i > 0 {
        println!("Exiting the terminal in {} seconds...", i);
        thread::sleep(Duration::from_secs(1));
        i -= 1;
    }
    process::exit(0);
}
