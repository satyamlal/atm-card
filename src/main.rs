mod atm;
mod pin;
mod utils;

use atm::{deposit_money, fetch_balance, withdraw_money};
use pin::{change_pin, login, read_pin};
use std::io;
use utils::exit_command;

fn main() {
    let mut user_balance: u64 = 0;
    let mut saved_pin: u64 = 8555;
    let mut attempt_count: u8 = 0;
    let mut pin_changed: bool = false;

    println!("-----------------------------------");
    println!("[INFO] Welcome to the ATM terminal.");

    login(saved_pin, &mut attempt_count);

    loop {
        if pin_changed {
            // Ask for re-verification if PIN was changed
            match read_pin("[INPUT] Please enter your PIN to proceed: ") {
                Some(entered_pin) if entered_pin == saved_pin => {
                    println!("[SUCCESS] PIN Verified. Access Granted!");
                    pin_changed = false;
                    continue;
                }
                Some(_) => {
                    attempt_count += 1;
                    if attempt_count >= 3 {
                        println!(
                            "[BLOCKED] 3 Wrong PIN attempt(s). Your account has been blocked for 48 hours."
                        );
                        exit_command();
                    }
                    eprintln!(
                        "[ERROR] Wrong PIN. {} attempt(s) remaining.",
                        3 - attempt_count
                    );
                    continue;
                }
                None => {
                    eprintln!("[ERROR] Invalid input. Please enter numbers only.");
                    continue;
                }
            }
        }

        println!("--------------------------------------------");
        println!("  [MENU] Please select your option.");
        println!("  1. Deposit Money");
        println!("  2. Withdraw Money");
        println!("  3. Check Balance");
        println!("  4. Change PIN");
        println!("  5. Exit");
        println!("--------------------------------------------");
        println!("[INPUT] Enter your choice: ");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("[FATAL] Unable to read 'user_input'!");

        let choice: u8 = match user_input.trim().parse() {
            Ok(choice) => choice,
            Err(_) => {
                eprintln!("[ERROR] Please enter a number between 1 to 5!");
                continue;
            }
        };

        if choice == 5 {
            exit_command();
        }

        if !(1..=5).contains(&choice) {
            eprintln!("[ERROR] Invalid option: {}. Choose between 1 to 5", choice);
            continue;
        }

        match choice {
            1 => deposit_money(&mut user_balance),
            2 => withdraw_money(&mut user_balance),
            3 => fetch_balance(&mut user_balance),
            4 => change_pin(&mut saved_pin, &mut pin_changed, &mut attempt_count),
            5 => exit_command(),
            _ => eprintln!("[ERROR] Unhandled choice '{}' in execute_choice.", choice),
        }
    }
}
