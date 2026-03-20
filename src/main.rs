use std::io;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    let mut user_balance: u64 = 0;
    let mut count: u8 = 0;

    loop {
        let mut user_input = String::new();
        println!("Please select your option.");
        println!("1. Deposit Money");
        println!("2. Withdraw Money");
        println!("3. Check Balance");
        println!("4. Change PIN");
        println!("5. Exit");
        println!("");
        println!("Enter your choice: ");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Unable to read terminal input!");

        let choice: u8 = match user_input.trim().parse() {
            Ok(choice) => choice,
            Err(_) => {
                eprintln!("Please enter a valid number!");
                continue;
            }
        };

        if choice == 5 {
            exit_command();
        }

        println!("Please enter your PIN to proceed!: ");
        let mut saved_pin: u64 = 8555;
        let mut user_pin_entry = String::new();

        io::stdin()
            .read_line(&mut user_pin_entry)
            .expect("Unable to read PIN from the terminal!");

        let user_pin: u64 = match user_pin_entry.trim().parse() {
            Ok(pin_verify) => pin_verify,
            Err(_) => {
                println!("Wrong input!");
                break;
            }
        };
        if user_pin == saved_pin {
            execute_choice(choice, &mut user_balance, &mut saved_pin);
        } else {
            count += 1;
            if count == 3 {
                println!(
                    "You have entered 3 wrong PIN. Your account has been blocked for 48 hours."
                );
                exit_command();
            }
            println!("You entered a wrong PIN. Try again!");
            continue;
        }
    }
}

fn execute_choice(choice: u8, user_balance: &mut u64, saved_pin: &mut u64) {
    match choice {
        1 => deposit_money(user_balance),
        2 => withdraw_money(user_balance),
        3 => fetch_balance(user_balance),
        4 => change_pin(saved_pin),
        5 => exit_command(),
        _ => eprintln!("Wrong Input!"),
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

fn change_pin(saved_pin: &mut u64) {
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
