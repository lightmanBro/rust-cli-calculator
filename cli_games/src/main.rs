use::std::io;

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        println!("\n=== CLI Games Menu ===");
        println!("1️⃣ Calculator");
        println!("2️⃣ Guessing Game");
        println!("3️⃣ Exit");
        println!("Choose an option (1–3):");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim(){
            "1" => calculator(),
            "2" => guessing_game(),
            "3" => {
                println!("Exiting the program. Goodbye!");
                break;
            },
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn calculator(){
    println!("\n🧮 Simple CLI Calculator");

    let mut first = String::new();
    println!("Enter first number");
    io::stdin().read_line(&mut first).expect("Failed to read input");
    let first : f64 = match first.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };
    let mut op = String::new();
    println!("Enter operator (+, -, *, /):");
    io::stdin().read_line(&mut op).expect("Failed to read input");
    let op = op.trim();

    let mut second= String::new();
    println!("Enter second number");
    io::stdin().read_line(&mut second).expect("Failed to read input");
    let second :f64 = match second.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    let result = match op {
        "+" => first + second,
        "-" => first - second,
        "*" => first * second,
        "/" =>{
            if second == 0.0 {
                println!("Cannot divide by zero!");
                return;
            }
            first / second
        }
        _ => {
             println!("Invalid operator!");
            return;
        }
    };
    println!("Result: {} {} {} = {}",first,op,second,result);
}       

fn guessing_game(){
    println!("\n=== Guessing Game ===");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("Please input your guess (1-100):");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number {} in {} attempts.", secret_number, attempts);
                break;
            }
        }
    }
}