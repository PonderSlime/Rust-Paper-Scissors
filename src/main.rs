use rand::Rng;
use std::fmt;

#[derive(Debug, PartialEq)]
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
impl Choice {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen::<i32>() {
            0 => Choice::Rust,
            1 => Choice::Paper,
            _ => Choice::Scissors,
        }
    }

    fn winner(self, other: Choice) -> Option<String> {
        match (self, other) {
            (Choice::Rust, Choice::Scissors) | (Choice::Scissors, Choice::Paper) | (Choice::Paper, Choice::Rust) => Some("You Win!".to_string()),
            _ => None
        }
    }
    fn loss(self) -> String {
        match self {
            Choice::Rust => "Paper beats Rust".to_string(),
            Choice::Paper => "Scissors beats Paper".to_string(),
            Choice::Scissors => "Rust beats Scissors".to_string(),
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

        println!("You choose: {:?}", user_choice);
        println!("Bot chose: {:?}", bot_choice);

        match (user_choice, bot_choice) {
            (Choice::Rust, Choice::Scissors) | (Choice::Scissors, Choice::Rust) | (Choice::Paper, Choice::Scissors) => {
                println!("{:?}", user_choice.loss());
            }
            (Choice::Rust, Choice::Scissors) | (Choice::Scissors, Choice::Paper) | (Choice::Paper, Choice::Rust) => {
                println!("{:?}", bot_choice.winner(user_choice));
            }
            (Choice::Rust, Choice::Rust) | (Choice::Scissors, Choice::Scissors) | (Choice::Paper, Choice::Paper) => {
                println!("{:?}", user_choice.tie());
            }
        }

        if let Some(winner) = bot_choice.winner(user_choice) {
            println!("You lost! The winner is: {}", winner);
        } else if user_choice == bot_choice {
            println!("It's a tie!");
        }
        
        println!("Do you want to play again? (yes/no)");
        let mut play_again = String::new();
        std::io::stdin().read_line(&mut play_again).expect("Failed to read line");

        if play_again.trim().to_lowercase() != "yes" {
            break;
        }
    }
}