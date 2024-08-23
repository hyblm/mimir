use xordle::{Answers, Clue, Clues, Outcome, Word, WORD_LEN};

pub mod xordle;

// #[cfg(test)]
// mod tests;

pub const DISPLAY: bool = true;
pub const MAX_ATTEMPTS: u8 = 50;

pub trait Solver {
    fn make_guess(&mut self) -> Option<Word>;
    fn judge_outcome(&mut self, guess: Word, outcome: Outcome, answers: &Answers);

    fn play(&mut self, answers: &mut Answers) -> Option<u8> {
        for attempts in 1..=MAX_ATTEMPTS {
            let guess = self.make_guess()?;
            let outcome = answers.compare_guess(guess);
            if DISPLAY {
                print!("{} {outcome}", std::str::from_utf8(&guess).unwrap());
            }
            self.judge_outcome(guess, outcome, answers);
            if answers.solved() {
                return Some(attempts);
            }
            //             assert!(
            //                 self.accepts(&std::str::from_utf8(&guess).unwrap()),
            //                 "guess wasn't in the dictionary"
            //             );
        }

        None
    }
}

pub fn solver() -> impl Solver {
    Simple::new()
}

struct Simple {
    remaining: Vec<Word>,
}

impl Simple {
    fn new() -> Self {
        let remaining = include!("../word_lists/words.in").to_vec();
        Self { remaining }
    }

    fn matches(
        solved: &[u8],
        mut present: [u8; 26],
        vocab: Word,
        guess: Word,
        clues: Clues,
    ) -> bool {
        for ((clue, letter), guess) in clues.iter().zip(vocab).zip(guess) {
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
            for (&mark, marked) in clues.iter().zip(guess) {
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

    fn judge_outcome(&mut self, guess: Word, outcome: Outcome, answers: &Answers) {
        if answers.solved() {
            return;
        }
        let (left, right) = answers.answers.split_at(WORD_LEN as usize);
        let solved = set_solved(guess, outcome.clues());
        let present = set_present(guess, outcome.clues());
        self.remaining.retain(|&vocab| {
            let eliminated = !Self::matches(&solved, present, vocab, guess, outcome.clues());
            assert!(
                !(eliminated && (vocab == left || vocab == right)),
                "\n\neliminated {}",
                std::str::from_utf8(&vocab).unwrap()
            );
            !eliminated
        });
        println!(" {} options left", self.remaining.len());
    }
}

fn set_solved(guess: [u8; 5], clues: Clues) -> [u8; 26] {
    let mut solved = [WORD_LEN; (b'z' - b'a' + 1) as usize];

    for (i, (mark, letter)) in clues.iter().zip(guess).enumerate() {
        if let Clue::Solved = mark {
            solved[(letter - b'a') as usize] = i as u8;
        }
    }

    solved
}

fn set_present(guess: [u8; 5], clues: Clues) -> [u8; 26] {
    let mut present = [0u8; (b'z' - b'a' + 1) as usize];

    for (clue, letter) in clues.iter().zip(guess) {
        if let Clue::Solved | Clue::Misput = clue {
            present[(letter - b'a') as usize] += 1;
        }
    }

    present
}
