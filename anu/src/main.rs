use std::io;

fn main() {
    let err_message = String::from("You must enter a valid number");
    let guess = input_u8(err_message);
    println!("{guess}");
}

fn input_u8(err_message: String) -> u8 {
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