pub mod solver;
pub mod xordle;

#[cfg(test)]
mod tests;

use solver::Solver;
use xordle::{
    hardcoded_games::{ANSWER_PAIRS, INITIAL_GUESSES},
    GameInstance,
};

const WORD_LEN: usize = 5;
const WORD_COUNT: usize = 2;
const ANSWERS_LEN: usize = WORD_COUNT * WORD_LEN;

pub fn play_all_games(display: bool) {
    for answers in ANSWER_PAIRS {
        let mut game = GameInstance::new(answers);
        let mut guesser = Solver::new();
        game.play(&mut guesser, display);
    }
}

pub fn play_all_games_with_initial_guess(display: bool) {
    for (&answers, &initial_guess) in ANSWER_PAIRS.iter().zip(INITIAL_GUESSES) {
        let mut game = GameInstance::new(answers);
        let mut guesser = Solver::new();
        if display {
            let score = game.rate_guess(initial_guess);
            println!("{} {}", std::str::from_utf8(&initial_guess).unwrap(), score);
            guesser.register_feedback(initial_guess, score);
        }
        game.play(&mut guesser, display);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Letter {
    Absent,
    Misput,
    Found,
}
pub type Feedback = [Letter; crate::WORD_LEN];

pub enum Target {
    Hit,
    Miss(Feedback),
}

impl Target {
    pub fn feedback(&self) -> Feedback {
        match self {
            Target::Hit => [Letter::Absent; 5],
            Target::Miss(feedback) => *feedback,
        }
    }
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Hit => write!(f, "answer found")?,
            Target::Miss(letters) => {
                for letter in letters {
                    match letter {
                        Letter::Absent => write!(f, "⬛")?,
                        Letter::Misput => write!(f, "🟨")?,
                        Letter::Found => write!(f, "🟩")?,
                    }
                }
            }
        }
        Ok(())
    }
}
