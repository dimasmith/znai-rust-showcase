//! This create is created to be a mere showcase for setting up Znai documentation.
//!
//! By coincidence, it also implements some calculations for the Elo rating system :)

/// The K-factor determines how much the rating changes after a game
const K_FACTOR: i32 = 32; // The K-factor determines how much the rating changes after a game

/// The outcome of a game of chess.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameOutcome {
    BlackWon,
    WhiteWon,
    Draw,
}

/// Calculates the new ratings for both players after a game.
///
/// # Arguments
/// - white_rating - The rating of the white pieces player before the game
/// - black_rating - The rating of the black pieces player before the game
/// - game_outcome - The outcome of the game
///
/// # Examples
/// ```
/// use znai_rust_showcase::{GameOutcome, calculate_new_ratings};
///
/// let (new_white_rating, new_black_rating) = calculate_new_ratings(
///     1000, 1200, GameOutcome::WhiteWon);
/// assert_eq!(new_white_rating, 1024);
/// assert_eq!(new_black_rating, 1176);
/// ```
pub fn calculate_new_ratings(
    white_rating: u32,
    black_rating: u32,
    game_outcome: GameOutcome,
) -> (u32, u32) {
    let (white_score, black_score) = match game_outcome {
        GameOutcome::WhiteWon => (1.0, 0.0),
        GameOutcome::BlackWon => (0.0, 1.0),
        GameOutcome::Draw => (0.5, 0.5),
    };
    let (white_elo_change, black_elo_change) =
        calculate_elo_change(white_rating, black_rating, white_score, black_score);
    (
        (white_rating as i32 + white_elo_change) as u32,
        (black_rating as i32 + black_elo_change) as u32,
    )
}

fn calculate_elo_change(
    white_rating: u32,
    black_rating: u32,
    white_score: f64,
    black_score: f64,
) -> (i32, i32) {
    let (white_expected_score, black_expected_score) =
        calculate_expected_scores(white_rating, black_rating);
    let white_elo_change = calculate_rating_change(white_expected_score, white_score);
    let black_elo_change = calculate_rating_change(black_expected_score, black_score);
    (white_elo_change, black_elo_change)
}

fn calculate_expected_scores(white_rating: u32, black_rating: u32) -> (f64, f64) {
    let white_expected_score =
        1.0 / (1.0 + 10.0_f64.powf((black_rating as f64 - white_rating as f64) / 400.0));
    let black_expected_score = 1.0 - white_expected_score;
    (white_expected_score, black_expected_score)
}

fn calculate_rating_change(expected_score: f64, score: f64) -> i32 {
    (K_FACTOR as f64 * (score - expected_score)).round() as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn calculate_new_ratings_of_equal_players() {
        assert_eq!(
            calculate_new_ratings(1000, 1000, GameOutcome::WhiteWon),
            (1016, 984)
        );
        assert_eq!(
            calculate_new_ratings(1000, 1000, GameOutcome::BlackWon),
            (984, 1016)
        );
        assert_eq!(
            calculate_new_ratings(1000, 1000, GameOutcome::Draw),
            (1000, 1000)
        );
    }

    #[test]
    fn calculate_new_ratings_of_different_players() {
        assert_eq!(
            calculate_new_ratings(1000, 1200, GameOutcome::WhiteWon),
            (1024, 1176)
        );
        assert_eq!(
            calculate_new_ratings(1000, 1200, GameOutcome::BlackWon),
            (992, 1208)
        );
        assert_eq!(
            calculate_new_ratings(1000, 1200, GameOutcome::Draw),
            (1008, 1192)
        );
    }

    #[test]
    fn games_of_equally_rated_players() {
        assert_eq!(calculate_elo_change(1000, 1000, 1.0, 0.0), (16, -16));
        assert_eq!(calculate_elo_change(1000, 1000, 0.5, 0.5), (0, 0));
        assert_eq!(calculate_elo_change(1000, 1000, 0.0, 1.0), (-16, 16));
    }

    #[test]
    fn games_of_differently_rated_players() {
        assert_eq!(calculate_elo_change(1000, 1200, 1.0, 0.0), (24, -24));
        assert_eq!(calculate_elo_change(1000, 1200, 0.5, 0.5), (8, -8));
        assert_eq!(calculate_elo_change(1000, 1200, 0.0, 1.0), (-8, 8));
    }
}
