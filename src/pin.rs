use crate::utils::exit_command;
use std::io;

pub fn read_pin(prompt: &str) -> Option<u64> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("unable to read from terminal!");

    match input.trim().parse() {
        Ok(val) => Some(val),
        Err(_) => {
            eprintln!("Invalid input! Please enter only numbers!");
            None
        }
    }
}

pub fn change_pin(saved_pin: &mut u64, pin_changed: &mut bool, attemp_count: &mut u8) {
    let Some(current_pin) = read_pin("Please enter your current pin: ") else {
        return;
    };

    if current_pin != *saved_pin {
        println!("Wrong PIN! PIN change cancelled.");
        return;
    }

    let Some(new_pin) = read_pin("Enter your pin: ") else {
        return;
    };

    *saved_pin = new_pin;
    *pin_changed = true;
    *attemp_count = 0;
    println!("PIN changed successfully!")
}

pub fn login(saved_pin: u64, attempt_count: &mut u8) {
    // Initial Login using PIN
    loop {
        match read_pin("[INPUT] Please enter your PIN to proceed: ") {
            Some(entered_pin) if entered_pin == saved_pin => {
                println!("[SUCCESS] PIN Verified. Access Granted!");
                break;
            }
            Some(_) => {
                *attempt_count += 1;
                if *attempt_count >= 3 {
                    println!("[BLOCKED] 3 wrong PIN attempts. Account blocked for 48 hours.");
                    exit_command();
                }
                eprintln!(
                    "[ERROR] Wrong PIN. {} attempt(s) remaining.",
                    3 - *attempt_count
                );
            }
            None => {
                eprintln!("[ERROR] Invalid input. Please enter numbers only.");
            }
        }
    }
}
