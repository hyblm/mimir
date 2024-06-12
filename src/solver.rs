use std::io;

use crate::{xordle::Word, Feedback, Target, WORD_LEN};

mod dictionary;
use dictionary::Dictionary;

pub struct Solver {
    remaining: Dictionary,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            remaining: Dictionary::new(),
        }
    }

    pub fn make_guess(&mut self) -> Word {
        let mut input_buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        loop {
            if let Ok(n) = stdin.read_line(&mut input_buffer) {
                if n == WORD_LEN + 1 {
                    if input_buffer.is_ascii() {
                        let mut guess = [0; WORD_LEN];
                        for (letter, byte) in guess.iter_mut().zip(input_buffer.bytes()) {
                            *letter = byte;
                        }
                        return guess;
                    } else {
                        println!("Guess contains non ASCII characters");
                    }
                } else {
                    println!("Guess needs to be {} letters", WORD_LEN);
                }
            }
        }
    }

    pub(crate) fn register_feedback(&mut self, guess: Word, feedback: Target) {

        // self.remaining.trim()
    }
}
