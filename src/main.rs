use std::fmt::Display;

use clap::{Parser, command, ValueEnum};
use znai_rust_showcase::{GameOutcome, calculate_new_ratings};


#[derive(Parser)]
#[command(author, about, version)]
struct Cli {
    white_rating: u32,
    black_rating: u32,
    #[arg(value_enum)]
    result: GameResult,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, ValueEnum)]
enum GameResult {
    Win,
    Loss,
    Draw,
}

fn main() {
    let args = Cli::parse();
    let white_rating = args.white_rating;
    let black_rating = args.black_rating;
    let result = args.result;
    let (new_white_rating, new_black_rating) =
        calculate_new_ratings(white_rating, black_rating, GameOutcome::from(result));
    println!("Before game:\t{white_rating}\t{black_rating}");
    println!("After game: \t{new_white_rating}\t{new_black_rating}");
}

impl Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameResult::Win => write!(f, "White won"),
            GameResult::Loss => write!(f, "Black won"),
            GameResult::Draw => write!(f, "Draw"),
        }
    }
}

impl From<GameResult> for GameOutcome {
    fn from(result: GameResult) -> Self {
        match result {
            GameResult::Win => GameOutcome::WhiteWon,
            GameResult::Loss => GameOutcome::BlackWon,
            GameResult::Draw => GameOutcome::Draw,
        }
    }
}
