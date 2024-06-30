use std::io;
use rand::Rng;

use std::cmp::Ordering;


fn main() {
    println!("Тут начинается игра УГАДАЙ ЧИСЛО!");

    let random_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Введите секретное число: ");

        let mut user_guesses_int_number = String::new();

        io::stdin()
            .read_line(&mut user_guesses_int_number)
            .expect("Там какая-то проблема с вводом числа...");
        

        let user_guesses_int_number: u32 = match user_guesses_int_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Случайное число: {random_number}");

   

   

        match user_guesses_int_number.cmp(&random_number){
            Ordering::Less => println!("Less..."),
            Ordering::Greater => println!("Greater..."),
            Ordering::Equal => {
                println!("Equal...BINGO!!!");
                break;
            }
        }
        
        println!("Ты угадал правильное число : {}", user_guesses_int_number);
        //let mut user_guesses_number : u32  = parse() 
        }
    

}











