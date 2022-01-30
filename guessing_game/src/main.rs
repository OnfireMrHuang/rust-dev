// 第一部分，允许用户操作
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Gueess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input you guess.");

        // let appple = 5; // 不可变变量，如果加上mut就是可变变量
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
             Ok(num) => num,
             Err(_) => continue,
        };

    
        println!("You guessed1: {}",guess);
    
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    
    }
}