use high_scores::*;

fn main() {
    let score_board = HighScores::new(&[30, 50, 20, 70]);
    score_board.scores();
}