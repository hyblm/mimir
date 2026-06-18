use std::io::{self, Write};

use crate::xordle::{Clue, Mark, Outcome, WORD_LEN, Word};
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

    fn matches(solved: &[u8], present: [u8; 26], vocab: Word, guess: Word, mark: Mark) -> bool {
        let mut counts = [0u8; 26];

        !vocab.into_iter().enumerate().any(|(i, letter)| {
            let letter_idx = (letter - b'a') as usize;
            counts[letter_idx] += 1;
            let solved_index = solved[letter_idx] as usize;

            let violates_green = solved_index < WORD_LEN && letter != vocab[solved_index];
            let violates_yellow = letter == guess[i] && mark[i] == Clue::Misput;

            violates_green || violates_yellow
        }) && !mark.iter().zip(guess).any(|(&mark, letter)| {
            let marked_idx = (letter - b'a') as usize;
            mark == Clue::Absent && counts[marked_idx] > present[marked_idx]
        })
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

pub struct Human;

impl Human {
    pub fn new() -> Self {
        Self
    }
}

impl Solver for Human {
    fn make_guess(&mut self) -> Option<Word> {
        loop {
            print!("Enter a 5-letter word: ");
            io::stdout().flush().ok()?;

            let mut input = String::new();
            if io::stdin().read_line(&mut input).ok()? == 0 {
                return None;
            }

            let guess = input.trim().to_ascii_lowercase();
            if guess == "quit" || guess == "exit" {
                return None;
            }

            let bytes = guess.as_bytes();
            if bytes.len() == WORD_LEN && bytes.iter().all(u8::is_ascii_lowercase) {
                return Some(bytes.try_into().unwrap());
            }

            eprintln!("Please enter exactly 5 ASCII letters, or `quit` to stop.");
        }
    }

    fn judge_outcome(&mut self, guess: Word, outcome: Outcome) {
        let (was_correct, mark) = outcome;

        for letter in str::from_utf8(&guess).unwrap().to_uppercase().chars() {
            print!("{letter} ");
        }
        println!();

        if was_correct {
            println!("🌠🌠🌠🌠🌠 Correct!");
        } else {
            for clue in mark {
                match clue {
                    Clue::Absent => print!("⬛"),
                    Clue::Misput => print!("🟨"),
                    Clue::Solved => print!("🟩"),
                }
            }
            println!();
        }
    }
}
