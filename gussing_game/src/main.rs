use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Guess the number!");

    let sec_key = rand::thread_rng().gen_range(1..100);
    println!("The secret key is {}", sec_key);

    loop {
        println!("Please enter your input");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("There is an error in the input. Ensure that you enter an integer.");
                continue;
            }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&sec_key) {
            Ordering::Less => println!("You guessed a lower number"),
            Ordering::Greater => println!("You guessed a higher number"),
            Ordering::Equal => {
                println!("CORRECT!");
                break;
            }
        }
    }
}




// fn main() {
//     println!("Guess the number!");

//     let sec_key = rand::thread_rng().gen_range(1..=100);

//     println!("the sec_key is {sec_key}");
//     loop {
//         println!("please input your guess");

//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read the line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {println!("Please input a number"); continue;}
//         };

//         println!("you guessed: {guess}");

//         match guess.cmp(&sec_key) {
//             Ordering::Less => println!("Too Small"),
//             Ordering::Greater => println!("Too big"),
//             Ordering::Equal => {
//                 println!("Guessed the number right"); break;
//             }
//         }
//     }
// }
