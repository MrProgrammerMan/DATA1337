use std::{cmp::Ordering, io};
use std::fmt::Display;

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

    let mut shopping_list: Vec<(String, i32)> = Vec::new();

    loop {
        present_menu(&MENU_NAME, &CHOICES);
        match bound(INVALID_MENU_CHOICE_MESSAGE, 0, CHOICES.len() as u8) {
            0 => return,
            1 => run_games_menu(),
            2 => run_shopping_list_menu(&mut shopping_list),
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
            2 => run_tic_tac_toe(),
            _ => unreachable!()
        }
    }
}

fn run_shopping_list_menu(shopping_list: &mut Vec<(String, i32)>) {
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
            1 => display_shopping_list(shopping_list),
            2 => add_to_shopping_list(shopping_list),
            3 => remove_from_shopping_list(shopping_list),
            _ => unreachable!()
        }
    }
}

fn display_shopping_list(shopping_list: &Vec<(String, i32)>) {
    clear_screen();
    println!("{}", shopping_list.iter()
        .map(|(item, i)| format!("Item: {}\tamount: {}", item, i))
        .collect::<Vec<String>>()
        .join("\n"));
    println!("Press ENTER to return");
    wait_for_input();
}

fn add_to_shopping_list(shopping_list: &mut Vec<(String, i32)>) {
    clear_screen();
    println!("Enter the name of the item to add:");
    let input = String::from(input_string().trim());
    const SHOPPING_LIST_AMOUNT_ERROR_MESSAGE: &'static str = "Enter a valid amount between 1 and 100";

    let existing = shopping_list.iter_mut().find(|(item, _)| *item == input);

    if let Some(item) = existing {
        println!("Item is already in the shopping list.");
        println!("Please enter the new amount: ");
        let amount = bound(SHOPPING_LIST_AMOUNT_ERROR_MESSAGE, 1, 101);
        item.1 = amount.into();
    } else {
        println!("Please enter the amount: ");
        let amount = bound(SHOPPING_LIST_AMOUNT_ERROR_MESSAGE, 1, 101);
        shopping_list.push((input, amount.into()));
    }
}

fn remove_from_shopping_list(shopping_list: &mut Vec<(String, i32)>) {
    clear_screen();
    println!("Enter the name of the item to remove:");
    let mut input = String::from(input_string().trim());

    input = String::from(input.trim());

    if let Some((i, _)) = shopping_list.iter_mut().enumerate().find(|(_, (item, _))| *item == input) {
        shopping_list.remove(i);
        println!("Item removed");
    } else {
        println!("No such item on the list");
    }
    println!("Press ENTER to return");
    wait_for_input();
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
    println!("Have a guess: ");
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
        println!("Have another guess: ");
        guess = bound(INVALID_GUESS_MESSAGE, 1, 101);
        remaining_tries -= 1;
    }
    println!("Press ENTER to return");
    wait_for_input();
}

fn run_tic_tac_toe() {
    let mut board = TicTacToe::new();
    let mut blank_spaces = board.get_blank_spaces();
    let mut turn = true;
    
    loop {
        clear_screen();
        println!("TIC-TAC-TOE");
        println!("{}", board);

        if let Some(winner) = board.won() {
            println!("The winner is {}!!!", winner);
            println!("Press ENTER to return");
            wait_for_input();
            return;
        }
        if blank_spaces.is_empty() {
            println!("The round ends in a draw...");
            println!("Press ENTER to return");
            wait_for_input();
            return;
        }

        let player = match turn {
            true => Field::X,
            false => Field::O
        };

        let move_to_make = match turn {
            true => get_tic_tac_toe_choice(&blank_spaces),
            false => rand::random_range(0..blank_spaces.len())
        };

        make_tic_tac_toe_move(&mut board, &mut blank_spaces, move_to_make, player);

        turn = !turn;
    }
}

fn make_tic_tac_toe_move(board: &mut TicTacToe, free_spaces: &mut Vec<usize>, move_to_make: usize, player: Field) {
    board.set_field(free_spaces[move_to_make], player);
    free_spaces.remove(move_to_make);
}

fn get_tic_tac_toe_choice(blank_spaces: &Vec<usize>) -> usize {
    const INVALID_CELL_MESSAGE: &'static str = "You must input a valid cell between 1 and 9";
    let input = (bound(INVALID_CELL_MESSAGE, 1, 10)-1) as usize;
    let free_space = blank_spaces
        .iter()
        .enumerate()
        .find(|(_, s)| **s == input);
    if let Some((free_space_index, _)) = free_space {
        free_space_index
    } else {
        println!("That space is not free. Choose another");
        get_tic_tac_toe_choice(blank_spaces)
    }
}

fn input_u8(err_message: &str) -> u8 {
    match input_string().trim().parse::<u8>() {
        Ok(n) => n,
        Err(_) => {
            println!("{}", err_message);
            input_u8(err_message)
        }
    }
}

fn input_string() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
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

struct TicTacToe {
    fields: [[Field; 3]; 3],
}

impl TicTacToe {
    fn new() -> Self {
        Self {
            fields: [[Field::Blank; 3]; 3]
        }
    }
    fn set_field(&mut self, i: usize, f: Field) {
        let row = i / 3;
        let col = i % 3;
        self.fields[row][col] = f;
    }
    fn get_blank_spaces(&self) -> Vec<usize> {
        self.fields
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_, f)| **f == Field::Blank)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
    }
    fn won(&self) -> Option<Field> {
        self.get_valid_lines().iter().find(|line| {
            let line_ref = line[0];
            if line_ref == &Field::Blank {
                false
            } else {
                line.iter().all(|f| *f == line_ref)
            }
        }).and_then(|row| Some(*row[0]))
    }
    fn get_valid_lines(&self) -> Vec<[&Field; 3]> {
        let mut lines = Vec::with_capacity(8);

        // rows
        for row in &self.fields {
            let [a, b, c] = row;
            lines.push([a, b, c]);
        }

        // columns
        for col in 0..3 {
            lines.push([
                &self.fields[0][col],
                &self.fields[1][col],
                &self.fields[2][col],
            ]);
        }

        // diagonals
        lines.push([
            &self.fields[0][0],
            &self.fields[1][1],
            &self.fields[2][2],
        ]);

        lines.push([
            &self.fields[0][2],
            &self.fields[1][1],
            &self.fields[2][0],
        ]);

        lines
    }
}

impl Display for TicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┌───┬───┬───┐")?;

        for (i, row) in self.fields.iter().enumerate() {
            write!(f, "│")?;
            for cell in row {
                write!(f, " {} │", cell)?;
            }
            writeln!(f)?;

            if i < self.fields.len() - 1 {
                writeln!(f, "├───┼───┼───┤")?;
            }
        }

        writeln!(f, "└───┴───┴───┘")
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Field::Blank => " ",
            Field::O => "○",
            Field::X => "✕"
        })
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Field {
    X,
    O,
    Blank
}