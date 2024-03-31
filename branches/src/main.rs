use std::io;
use rand::Rng;

fn main() {
  let secret_number: u32 = rand::thread_rng().gen_range(1..100);
  println!("The secret number is {}", secret_number);

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
          Ok(_)=> {
            let input:u32 = match input.trim().parse() {
              Ok(num) => num,
              Err(_)=> {
                println!("please enter a valid Input"); continue; 
              }
            };
            if input == secret_number {
              println!("Match"); break;
            }
            if input> secret_number {
              println!("input greater");
            } else {
              println!("Input is smaller");}
          }
          Err(_)=> {
            println!("Please Enter a valid input");
            break;
          }
        }
    }
}
