use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("{}", "🎮 Welcome to CLI Games!".bright_blue().bold());
    println!("{}", "-------------------------".bright_blue());

    loop {
        println!("\n{}", "=== Main Menu ===".bright_yellow().bold());
        println!("{}", "1️⃣  Calculator".green());
        println!("{}", "2️⃣  Guessing Game".cyan());
        println!("{}", "3️⃣  Exit".red());
        println!("{}", "Choose an option (1–3):".yellow());

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => calculator(),
            "2" => guessing_game(),
            "3" => {
                println!("{}", "👋 Exiting... Goodbye!".bright_red().bold());
                break;
            }
            _ => println!("{}", "❌ Invalid choice! Try again.".red()),
        }
    }
}

fn calculator() {
    println!("\n{}", "🧮 Simple CLI Calculator".bright_green().bold());

    let first = prompt_number("Enter first number:");
    let op = prompt_operator("Enter operator (+, -, *, /):");
    let second = prompt_number("Enter second number:");

    let result = match op.as_str() {
        "+" => first + second,
        "-" => first - second,
        "*" => first * second,
        "/" => {
            if second == 0.0 {
                println!("{}", "⚠️ Cannot divide by zero!".red());
                return;
            }
            first / second
        }
        _ => {
            println!("{}", "❌ Invalid operator!".red());
            return;
        }
    };

    println!(
        "{} {} {} {} {}",
        first.to_string().bright_cyan(),
        op.bright_yellow(),
        second.to_string().bright_cyan(),
        "=".bright_yellow(),
        result.to_string().bright_green().bold()
    );
}

fn guessing_game() {
    println!("\n{}", "🎯 Welcome to the Guessing Game!".bright_cyan().bold());
    println!("{}", "I'm thinking of a number between 1 and 100...".bright_black());

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("{}", "Enter your guess:".yellow());
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "⚠️ Please enter a valid number!".red());
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small! 📉".blue()),
            Ordering::Greater => println!("{}", "Too big! 📈".magenta()),
            Ordering::Equal => {
                println!(
                    "{} {} {}",
                    "🎉 You got it!".bright_green().bold(),
                    format!("The number was {}", secret_number).bright_yellow(),
                    format!("(Attempts: {})", attempts).bright_black()
                );
                break;
            }
        }
    }
}

// Helper: read and parse number input
fn prompt_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt.bright_white());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("{}", "❌ Invalid number! Try again.".red()),
        }
    }
}

// Helper: read operator
fn prompt_operator(prompt: &str) -> String {
    println!("{}", prompt.bright_white());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
