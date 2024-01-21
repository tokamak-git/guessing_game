// add imports for this program
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("You have 3 chances to guess the number.");

    let secret_number = rand::thread_rng().gen_range(1..11);
    let mut counter = 1;
    loop {
        let mut guess = String::new();
        println!("The secret number is: {}", secret_number);
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        if counter == 3 {
            println!("You lose!");
            break;
        }

        println!("You guessed: {}", guess);
        counter += 1;
    }
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
