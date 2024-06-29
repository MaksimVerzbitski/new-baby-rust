use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main(){
    println!("Guess the number within 1..= 100...");

    let guess_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your number: {guess_number}");

        let mut guessed_number = String::new();

        //let mut _number_guessed = string::new();

        io::stdin()
            .read_line(&mut guessed_number)
            .expect("Error message here...");
    
        let guessed_number: u32 = match guessed_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You have guessed number: {guessed_number}");

        match guessed_number.cmp(&guess_number){
            Ordering::Less => println!("Malenjkij nomer"),
            Ordering::Greater => println!("Bolshoi"),
            Ordering::Equal => {
                println!("Bingo!!!!");
                break;
            }
        }
    }
}



