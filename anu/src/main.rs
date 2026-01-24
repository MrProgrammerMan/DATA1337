use std::{cmp::Ordering, io};

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
            2 => run_shopping_list_menu(),
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
            1 => run_guess_the_number(),
            2 => println!("Not yet implemented"),
            _ => unreachable!()
        }
    }
}

fn run_shopping_list_menu() {
    const MENU_NAME: &'static str = "SHOPPING LIST";
    const CHOICES: [&'static str; 4] = [
        "Main menu",
        "Show shopping list",
        "Add to shopping list",
        "Remove from shopping list(by name)"
    ];

    loop {
        present_menu(&MENU_NAME, &CHOICES);
        match bound(INVALID_MENU_CHOICE_MESSAGE, 0, CHOICES.len() as u8) {
            0 => return,
            1 => println!("Not yet implemented"),
            2 => println!("Not yet implemented"),
            3 => println!("Not yet implemented"),
            _ => unreachable!()
        }
    }
}

fn run_guess_the_number() {
    let secret_number = rand::random_range(1..=100) as u8;
    let mut remaining_tries = 8u8;
    const INVALID_GUESS_MESSAGE: &'static str = "You must input a valid guess between 1 and 100";

    clear_screen();
    println!("Welcome to Guess The Number!");
    println!("The number is between 1 and 100");
    println!("You have {} tries to guess the number", remaining_tries);
    remaining_tries -= 1;
    print!("Have a guess: ");
    let mut guess = bound(INVALID_GUESS_MESSAGE, 1, 101);

    loop {
        clear_screen();
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} is lower than the secret number.", guess),
            Ordering::Greater => println!("{} is higher than the secret number.", guess),
            Ordering::Equal => {
                println!("{} is the correct number! You WIN!!!", guess);
                println!("You guessed the number with {} tries left", remaining_tries);
                break;
            }
        }
        if remaining_tries <= 0 {
            println!("Unfortunately you failed to guess the numbes");
            println!("Better luck next time!");
            break;
        }
        println!("You have {} tries left to guess the number", remaining_tries);
        print!("Have another guess: ");
        guess = bound(INVALID_GUESS_MESSAGE, 1, 101);
        remaining_tries -= 1;
    }
    println!("Press ENTER to return");
    wait_for_input();
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
    clear_screen();
    println!("{}", name);
    for (i, item) in choices.iter().enumerate() {
        println!("{}. {}", i, item);
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}

fn wait_for_input() {
    let mut _s = String::new();
    io::stdin()
        .read_line(&mut _s)
        .expect("Failed to read input");
}