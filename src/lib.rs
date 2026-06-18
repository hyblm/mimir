use crate::xordle::{Clue, Mark, WORD_LEN, Word};

pub mod xordle;

pub mod console {
    #![allow(unused)]

    pub const CLEAR: &str = "\x1B[J";
    pub const RESET_CURSOR: &str = "\x1B[0;0H";
    pub const GREEN_BG: &str = "\x1B[102m";
    pub const RED_BG: &str = "\x1B[101m";
    pub const BOLD: &str = "\x1B[1m";
    pub const YELLOW_BG: &str = "\x1B[103m";
    pub const BLACK_FG: &str = "\x1B[30m";
    pub const RESET_COLORS: &str = "\x1B[0m";
}
// #[cfg(test)]
// mod tests;

pub const DISPLAY: bool = false;
pub const MAX_ATTEMPTS: u8 = 50;

type Outcome = (bool, Mark);
pub trait Solver {
    fn make_guess(&mut self) -> Option<Word>;
    fn judge_outcome(&mut self, guess: Word, outcome: Outcome);
}

pub struct Simple {
    remaining: Vec<Word>,
}

impl Simple {
    pub fn new() -> Self {
        let remaining = include!("../word_lists/words.in").to_vec();
        Self { remaining }
    }

    fn matches(solved: &[u8], mut present: [u8; 26], vocab: Word, guess: Word, mark: Mark) -> bool {
        for ((clue, letter), guess) in mark.iter().zip(vocab).zip(guess) {
            let solved_index = solved[(letter - b'a') as usize] as usize;
            if solved_index < WORD_LEN as usize && letter != vocab[solved_index] {
                return false;
            }
            if letter == guess {
                if let Clue::Misput = clue {
                    return false;
                }
            }
        }

        for letter in vocab {
            for (&mark, marked) in mark.iter().zip(guess) {
                if mark != Clue::Absent {
                    continue;
                }
                if letter == marked {
                    if present[(letter - b'a') as usize] == 0 {
                        return false;
                    }
                    present[(letter - b'a') as usize] -= 1;
                    break;
                }
            }
        }

        true
    }
}

impl Solver for Simple {
    fn make_guess(&mut self) -> Option<Word> {
        self.remaining.pop()
    }

    fn judge_outcome(&mut self, guess: Word, outcome: Outcome) {
        let solved = set_solved(guess, outcome.1);
        let present = set_present(guess, outcome.1);
        self.remaining
            .retain(|&x| Self::matches(&solved, present, x, guess, outcome.1));
        // println!(" {} options left", self.remaining.len());
    }
}

fn set_solved(guess: [u8; 5], mark: Mark) -> [u8; 26] {
    let mut solved = [WORD_LEN as u8; (b'z' - b'a' + 1) as usize];

    for (i, (mark, letter)) in mark.iter().zip(guess).enumerate() {
        if let Clue::Solved = mark {
            solved[(letter - b'a') as usize] = i as u8;
        }
    }

    solved
}

fn set_present(guess: [u8; 5], clues: Mark) -> [u8; 26] {
    let mut present = [0u8; (b'z' - b'a' + 1) as usize];

    for (clue, letter) in clues.iter().zip(guess) {
        if let Clue::Solved | Clue::Misput = clue {
            present[(letter - b'a') as usize] += 1;
        }
    }

    present
}
