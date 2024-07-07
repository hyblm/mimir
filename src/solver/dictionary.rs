use crate::{xordle::Word, Feedback, Letter, WORD_LEN};

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

    pub fn trim(&mut self, guess: Word, feedback: Feedback) {
        let mut rejected_words = Vec::new();

        'words: for (i, word) in self.words.iter().enumerate() {
            for (position, letter) in word.iter().enumerate() {
                for (p, info) in feedback.iter().enumerate() {
                    match info {
                        Letter::Absent => {
                            rejected_words.push(i);
                            continue 'words;
                        }
                        Letter::Misput if position == p => {
                            rejected_words.push(i);
                            continue 'words;
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}
