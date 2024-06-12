pub mod hardcoded_games;

use crate::{solver::Solver, Letter, Target, ANSWERS_LEN, WORD_COUNT, WORD_LEN};

pub type Word = [u8; 5];

// Xordle responses
// two words remain
// that's not in the word list
// you got {answer}, one more to go
pub struct GameInstance {
    targets: [u8; ANSWERS_LEN],
    start: usize,
    end: usize,
}

/// A collection of all the words accepted by Xordle as valid guesses
static DICTIONARY: quickphf::PhfMap<&str, usize> = include!(concat!(env!("OUT_DIR"), "/dict.rs"));

const MAX_ATTEMPTS: u8 = 100;

impl GameInstance {
    pub fn new(target_strs: [&'static str; WORD_COUNT]) -> Self {
        let mut targets: [u8; ANSWERS_LEN] = [0; ANSWERS_LEN];
        for (target_byte, byte) in targets
            .iter_mut()
            .zip(target_strs.iter().flat_map(|&str| str.bytes()))
        {
            *target_byte = byte;
        }
        Self {
            targets,
            start: 0,
            end: ANSWERS_LEN,
        }
    }

    pub fn hardcoded(index: usize) -> Self {
        Self::new(hardcoded_games::get(index))
    }

    pub fn play(&mut self, guesser: &mut Solver, display: bool) {
        for _ in 0..MAX_ATTEMPTS {
            let guess = guesser.make_guess();
            assert!(
                self.accepts(&std::str::from_utf8(&guess).unwrap()),
                "guess wasn't in the dictionary"
            );

            let score = self.rate_guess(guess);
            if display {
                println!("{} {}", std::str::from_utf8(&guess).unwrap(), score);
            }
            match score {
                Target::Hit => {
                    if display {
                        println!("That's an answer!")
                    }
                }
                Target::Miss(feedback) => {}
            }
        }
    }

    pub fn accepts(&self, guess: &str) -> bool {
        DICTIONARY.contains_key(guess)
    }

    pub fn rate_guess(&mut self, guess: Word) -> Target {
        assert_eq!(guess.len(), WORD_LEN);

        if guess == self.targets[..5] {
            self.start = 6;
            return Target::Hit;
        }
        if guess == self.targets[6..] {
            self.end = 6;
            return Target::Hit;
        }

        use Letter::*;
        let mut feedback = [Absent; WORD_LEN];
        let mut used = [false; ANSWERS_LEN];

        for answer_index in self.start..self.end {
            if guess[answer_index % WORD_LEN] == self.targets[answer_index] {
                feedback[answer_index % WORD_LEN] = Found;
                used[answer_index] = true;
            }
        }
        for letter_index in 0..WORD_LEN {
            if feedback[letter_index] == Found {
                continue;
            }
            for answer_index in self.start..self.end {
                if used[answer_index] {
                    continue;
                }
                if guess[letter_index] == self.targets[answer_index] {
                    feedback[letter_index] = Misput;
                    used[answer_index] = true;
                }
            }
        }
        Target::Miss(feedback)
    }

    pub fn print_answers(&self) {
        println!("[{} {}]", self.targets[0], self.targets[1]);
    }
}
