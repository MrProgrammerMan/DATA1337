use std::io;

const INVALID_MENU_CHOICE_MESSAGE: &'static str = "You must enter a valid menu option";

fn main() {
    run_main_menu();
}

fn run_main_menu() {
    const MENU_NAME: &'static str = "MAIN MENU";
    const CHOICES: [&'static str; 3] = [
        "Quit",
        "Games",
        "Shopping list"
    ];

    loop {
        present_menu(&MENU_NAME, &CHOICES);
        match bound(INVALID_MENU_CHOICE_MESSAGE, 0, CHOICES.len() as u8) {
            0 => return,
            1 => run_games_menu(),
            2 => println!("Option 2"),
            _ => unreachable!()
        }
    }
}

fn run_games_menu() {
    const MENU_NAME: &'static str = "GAMES MENU";
    const CHOICES: [&'static str; 3] = [
        "Main menu",
        "Guess the number",
        "Tic-Tac-Toe"
    ];

    loop {
        present_menu(&MENU_NAME, &CHOICES);
        match bound(INVALID_MENU_CHOICE_MESSAGE, 0, CHOICES.len() as u8) {
            0 => return,
            1 => println!("Not yet implemented"),
            2 => println!("Not yet implemented"),
            _ => unreachable!()
        }
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

fn present_menu(name: &str, choices: &[&str]) {
    print!("\x1B[2J");
    println!("{}", name);
    for (i, item) in choices.iter().enumerate() {
        println!("{}. {}", i, item);
    }
}