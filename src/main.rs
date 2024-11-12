use rand::Rng;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
enum Choice {
    Rust,
    Paper,
    Scissors,
}
impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Choice::Rust => write!(f, "rust"),
            Choice::Paper => write!(f, "paper"),
            Choice::Scissors => write!(f, "scissors"),
        }
    }
}
impl Choice { /* Issue here! won't regen at all, always scissors 😭 */
    fn new() -> Self { // Problem: this uses  a random number seeded by the system. It won't change for awhile, AND if the number is out of range it will always land on scissors. You need a random function with a range from 0 to 2
        match rand::thread_rng().gen_range(0..2) {
            0 => Choice::Rust,
            1 => Choice::Paper,
            _ => Choice::Scissors,
        }
    }

    fn winner(self, other: Choice) -> String {
        match other {
            Choice::Rust => "You won with Rust!".to_string(),
            Choice::Paper => "You won with Paper!!".to_string(),
            Choice::Scissors => "You won with Scissors!".to_string(),
        }
    }
    fn loss(self) -> String {
        match self {
            Choice::Rust => "Bot beats Rust. You lost!".to_string(),
            Choice::Paper => "Bot beats Paper. You lost!".to_string(),
            Choice::Scissors => "Bot beats Scissors. You lost!".to_string(),
        }
    }

    fn tie(self) -> String {
        "It's a tie!".to_string()
    }
}
fn main() {
    loop {
        println!("Enter your choice (rust, paper, or scissors):");
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).expect("Failed to read line!");
        let user_choice: Choice = match user_input.trim().to_lowercase().as_str() {
            "rust" => Choice::Rust,
            "paper" => Choice::Paper,
            "scissors" => Choice::Scissors,
            _ => panic!("Invalid input!"),
        };
        let bot_choice = Choice::new();
        println!("You chose: {:?}", user_choice);
        println!("Bot chose: {:?}", bot_choice);

        match (user_choice.clone(), bot_choice.clone()) {
            (Choice::Scissors, Choice::Rust) | (Choice::Rust, Choice::Paper) | (Choice::Paper, Choice::Scissors) => {
                println!("{:?}", user_choice.clone().loss());
            }
            (Choice::Rust, Choice::Scissors) | (Choice::Scissors, Choice::Paper) | (Choice::Paper, Choice::Rust) => {
                println!("{:?}", bot_choice.clone().winner(user_choice));
            }
            (Choice::Rust, Choice::Rust) | (Choice::Scissors, Choice::Scissors) | (Choice::Paper, Choice::Paper) => {
                println!("{:?}", user_choice.clone().tie());
            }
        }
        
        println!("Do you want to play again? (yes/no)");
        let mut play_again = String::new();
        std::io::stdin().read_line(&mut play_again).expect("Failed to read line");

        if play_again.trim().to_lowercase() != "yes" {
            break;
        }
    }
}