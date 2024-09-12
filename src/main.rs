use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");

    loop{
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("The secret number is: {secret_number}");
        let mut attempts: i32 = 1;

        loop {
            println!("Please input your guess");
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            
            let guess: u32 = match guess
                .trim()
                .parse(){
                    Ok(num) => num,
                    Err(_) => continue,
                };
    
            println!("You guessed: {}", guess);
    
            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    attempts += 1;
                    println!("Too small!");
                }
                Ordering::Greater => {
                    attempts += 1;
                    println!("Too big!");
                }
                Ordering::Equal => {
                    println!("You win!");
                    println!("you guessed the number in {attempts} attempts");
                    break;
                }
            }
        }
        let replay = ask_confirm("Play again? (y/n): ");
        if replay == false{
            break;
        }

    }
}

fn ask_confirm(question: &str) -> bool {
    println!("{}", question);
    loop {

        let mut s = String::new();
        // Read line and handle potential errors
        match io::stdin().read_line(&mut s) {
            Ok(_) => {
                // Trim the newline character and process input
                let s = s.trim();
                match s.chars().next() {
                    Some('y') | Some('Y') => return true,
                    Some('n') | Some('N') => return false,
                    _ => println!("y/n only please."),
                }
            }
            Err(_) => println!("Failed to read input. Please try again."),
        }
    }
}
