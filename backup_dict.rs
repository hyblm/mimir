use core::panic;

use crate::{xordle::Word, Feedback, Letter, WORD_LEN};
pub const _DECISION_CAP: usize = 50;

/// We have 12 947 words with frequency data from the google 5-gram data set
const DICTIONARY_SIZE: usize = 12_947;

pub struct Dictionary {
    pub words: Vec<[u8; WORD_LEN]>,
    pub counts: Vec<u64>,
}

impl Dictionary {
    pub fn new() -> Self {
        let words_array = include!("words.in");
        let mut words = Vec::with_capacity(DICTIONARY_SIZE);
        words.extend_from_slice(&words_array);

        let counts_array: [u64; DICTIONARY_SIZE] = include!("counts.in");
        let mut counts = Vec::with_capacity(DICTIONARY_SIZE);
        counts.extend_from_slice(&counts_array);

        Self { words, counts }
    }

    #[inline(always)]
    pub fn match_count(&self, guess: Word, feedback: Feedback) -> u64 {
        let mut used: [bool; 5] = [false; WORD_LEN];
        self.words
            .iter()
            // .take(DECISION_CAP)
            .filter(|&word| matches(&mut used, *word, guess, feedback))
            .count()
            .try_into()
            .expect("match count fits in a u64")
    }

    pub fn match_count_occurence_sum(&self, guess: Word, feedback: Feedback) -> u64 {
        let mut used: [bool; 5] = [false; WORD_LEN];
        self.words
            .iter()
            .zip(&self.counts)
            .filter_map(|(word, count)| matches(&mut used, *word, guess, feedback).then_some(count))
            .sum()
    }

    pub fn trim(&mut self, guess: Word, feedback: Feedback) {
        let mut rejected_indeces = Vec::new();
        let mut used: [bool; 5] = [false; WORD_LEN];

        let solved = set_solved(guess, feedback);
        let present = set_present(guess, feedback);
        for (index, &word) in self.words.iter().enumerate() {
            let v1 = matches_v1(&mut used, word, guess, feedback);
            let v2 = matches_v2(&solved, present.clone(), word, guess, feedback);
            // if v1 != v2 && guess != *b"state" {
            //     println!(
            //         "{}  - old {}  new {}",
            //         std::str::from_utf8(&word).unwrap(),
            //         v1,
            //         v2
            //     );
            //     assert_eq!(v1, v2);
            // }
            if !v2 {
                rejected_indeces.push(index);
            }
        }

        for index in rejected_indeces.into_iter().rev() {
            self.words.remove(index);
            self.counts.remove(index);
        }
    }
}

fn set_solved(guess: [u8; 5], feedback: Feedback) -> [u8; 26] {
    let mut solved = [WORD_LEN as u8; (b'z' - b'a' + 1) as usize];

    for (i, (mark, letter)) in feedback.iter().zip(guess).enumerate() {
        if let Letter::Solved = mark {
            solved[(letter - b'a') as usize] = i as u8;
        }
    }

    solved
}

fn set_present(guess: [u8; 5], feedback: Feedback) -> [u8; 26] {
    let mut present = [0u8; (b'z' - b'a' + 1) as usize];

    for (mark, letter) in feedback.iter().zip(guess) {
        if let Letter::Solved | Letter::Misput = mark {
            present[(letter - b'a') as usize] += 1;
        }
    }

    present
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

// fn matches_v3

fn matches_v2(
    solved: &[u8],
    mut present: [u8; 26],
    word: Word,
    guess: Word,
    feedback: Feedback,
) -> bool {
    for ((mark, letter), guess) in feedback.iter().zip(word).zip(guess) {
        let solved_index = solved[(letter - b'a') as usize] as usize;
        if solved_index < WORD_LEN {
            if letter != word[solved_index] {
                return false;
            }
        }
        if letter == guess {
            if let Letter::Misput = mark {
                return false;
            }
        }
    }

    for letter in word {
        for (&mark, marked) in feedback.iter().zip(guess) {
            if mark != Letter::Absent {
                continue;
            }
            if letter == marked {
                if present[(letter - b'a') as usize] == 0 {
                    return false;
                } else {
                    present[(letter - b'a') as usize] -= 1;
                    break;
                }
            }
        }
    }

    true
}

fn matches_v1(used: &mut [bool], word: Word, guess: Word, feedback: Feedback) -> bool {
    used.fill(false);

    for (((used, mark), letter), guess) in used.iter_mut().zip(feedback).zip(word).zip(guess) {
        if letter == guess {
            match mark {
                Letter::Misput => return false,
                Letter::Solved => *used = true,
                _ => (),
            }
        }
    }

    for (&mark, guess) in feedback.iter().zip(guess) {
        for (used, letter) in used.iter_mut().zip(word) {
            if *used {
                continue;
            }
            if letter == guess {
                match mark {
                    Letter::Absent => return false,
                    Letter::Misput => {
                        *used = true;
                    }
                    _ => (),
                }
            }
        }
    }

    true
}

fn matches(used: &mut [bool], word: Word, guess: Word, feedback: Feedback) -> bool {
    matches_v1(used, word, guess, feedback)
}
