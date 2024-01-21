// add imports for this program
// use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    // guess();
}

// fn guess() {
//     // generating a rangom number between 1 and 10
//     let secret_number = rand::thread_rng().gen_range(1..11);
//     // take user input
//     let mut guess = String::new();
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
//     let guess: u32 = guess.trim().parse().expect("Please type a number!");
//     println!("You guessed: {}", guess);
//     println!("The secret number is: {}", secret_number);
// }
