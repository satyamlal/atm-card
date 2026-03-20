use std::io;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    let mut user_balance: u64 = 0;
    let mut saved_pin: u64 = 8555;
    let mut attempt_count: u8 = 0;
    let mut pin_changed: bool = false;

    println!("-----------------------------------");
    println!("[INFO] Welcome to the ATM terminal.");

    // Initial Login using PIN
    loop {
        match read_pin("[INPUT] Please enter your PIN to proceed: ") {
            Some(entered_pin) if entered_pin == saved_pin => {
                println!("[SUCCESS] PIN Verified. Access Granted!");
                break;
            }
            Some(_) => {
                attempt_count += 1;
                if attempt_count >= 3 {
                    println!(
                        "[BLOCKED] 3 Wrong PIN attempt(s). Your account has been blocked for 48 hours."
                    );
                    exit_command();
                }
                println!(
                    "[ERROR] Wrong PIN. {} attempts remaining.",
                    3 - attempt_count
                );
            }
            None => {
                eprintln!("[ERROR] Invalid input. Please enter numbers only.");
            }
        }
    }

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
                        "[ERROR] Wrong PIN. {} attempts remaining.",
                        3 - attempt_count
                    );
                    continue;
                }
                None => {
                    eprintln!("[ERROR] Invalid input. Please enter numbers only.");
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
        execute_choice(
            choice,
            &mut user_balance,
            &mut saved_pin,
            &mut pin_changed,
            &mut attempt_count,
        );
    }
}

fn execute_choice(
    choice: u8,
    user_balance: &mut u64,
    saved_pin: &mut u64,
    pin_changed: &mut bool,
    attemp_count: &mut u8,
) {
    match choice {
        1 => deposit_money(user_balance),
        2 => withdraw_money(user_balance),
        3 => fetch_balance(user_balance),
        4 => change_pin(saved_pin, pin_changed, attemp_count),
        5 => exit_command(),
        _ => eprintln!("[ERROR] Unhandled choice '{}' in execute_choice.", choice),
    }
}

fn read_pin(prompt: &str) -> Option<u64> {
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

fn change_pin(saved_pin: &mut u64, pin_changed: &mut bool, attemp_count: &mut u8) {
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

fn deposit_money(user_balance: &mut u64) {
    println!("You current balance is : {}", user_balance);

    let mut dep_amnt_str = String::new();
    println!("Enter the amount you want to deposit: ");
    io::stdin()
        .read_line(&mut dep_amnt_str)
        .expect("Unable to read terminal input!");

    let dep_amnt: u64 = match dep_amnt_str.trim().parse() {
        Ok(val) if val > 0 => val,
        Ok(_) => {
            eprintln!("Deposit amount must be greater than zero!");
            return;
        }
        Err(_) => {
            eprintln!("Wrong terminal input!");
            return;
        }
    };

    if dep_amnt % 500 == 0 {
        *user_balance += dep_amnt;
        println!("Deposit successfull. Your New balance: {}", user_balance);
    } else {
        println!("Please enter the deposit amount in the denomination of 500.");
    }
}

fn withdraw_money(user_balance: &mut u64) {
    if *user_balance <= 0 {
        println!("You don't have enough balance to withdraw.");
        println!("Your current balance: {}", user_balance);
        return;
    }

    println!("You current balance is : {}", user_balance);
    println!("Please enter the withdrawal amount: ");
    let mut withdraw_amnt_str = String::new();

    io::stdin()
        .read_line(&mut withdraw_amnt_str)
        .expect("Unable to read terminal input!");

    let withdraw_amnt: u64 = match withdraw_amnt_str.trim().parse() {
        Ok(val) if val > 0 => val,
        Ok(_) => {
            println!("Withdrawal amount must be greater than zero!");
            return;
        }
        Err(_) => {
            eprintln!("Wrong input!");
            return;
        }
    };

    if withdraw_amnt <= *user_balance && withdraw_amnt % 500 == 0 {
        *user_balance -= withdraw_amnt;
        println!("Withdrawal successfull.");
        println!("You new updated balance is: {}", *user_balance);
        return;
    } else if withdraw_amnt % 500 != 0 {
        println!("Please enter the withdrawal amount in the denomination of 500.");
        return;
    } else {
        println!("You don't have enough balance!");
        return;
    }
}

fn fetch_balance(user_balance: &u64) {
    println!("You current balance is : {}", user_balance);
}

fn exit_command() {
    let mut i: u64 = 3;
    while i > 0 {
        println!("Exiting the terminal in {} seconds...", i);
        thread::sleep(Duration::from_secs(1));
        i -= 1;
    }
    process::exit(0);
}
