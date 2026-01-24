use std::io;

const INVALID_MENU_CHOICE_MESSAGE: &'static str = "You must enter a valid menu option";

fn main() {
    run_main_menu();
}

fn run_main_menu() {
    const CHOICES: [&'static str; 3] = [
        "Quit",
        "Games",
        "Shopping list"
    ];

    match bound(INVALID_MENU_CHOICE_MESSAGE, 0, CHOICES.len() as u8) {
        0 => println!("Option 0"),
        1 => println!("Option 1"),
        2 => println!("Option 2"),
        _ => unreachable!()
    }
}

fn input_u8(err_message: &str) -> u8 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<u8>() {
        Ok(n) => n,
        Err(_) => {
            println!("{}", err_message);
            input_u8(err_message)
        }
    }
}

fn bound(err_message: &str, lower_bound: u8, upper_bound: u8) -> u8 {
    let n = input_u8(err_message);
    if n >= lower_bound && n < upper_bound {
        n
    } else {
        println!("{}", err_message);
        bound(err_message, lower_bound, upper_bound)
    }
    
}