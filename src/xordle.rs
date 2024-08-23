// use crate::{Clue, Outcome};

pub const WORD_LEN: u8 = 5;
pub const WORD_COUNT: u8 = 2;
pub const ANSWERS_LEN: u8 = WORD_COUNT * WORD_LEN;
pub type Word = [u8; 5];
pub fn word(slice: &[u8]) -> Word {
    [slice[0], slice[1], slice[2], slice[3], slice[4]]
}

// Xordle responses
#[repr(transparent)]
pub struct Answers<'a> {
    pub answers: &'a [u8],
}

/// A collection of all the words accepted by Xordle as valid guesses
// static DICTIONARY: quickphf::PhfMap<&str, usize> = include!(concat!(env!("OUT_DIR"), "/dict.rs"));

impl<'a> Answers<'a> {
    pub fn new(answers: &'a [u8]) -> Self {
        Self { answers }
    }
    pub fn solved(&self) -> bool {
        self.answers.is_empty()
    }

    pub fn compare_guess(&mut self, guess: Word) -> Outcome {
        let (left, right) = self.answers.split_at(WORD_LEN as usize);
        if guess == left {
            self.answers = right;
            return Outcome(None);
        }
        if guess == right {
            self.answers = left;
            return Outcome(None);
        }
        // a LUT of how many times a given letter was contained in the answer but not in guess
        let mut unsolved_letters = [0u8; (b'z' - b'a' + 1) as usize];
        let mut clues = [Clue::Absent; WORD_LEN as usize];

        for answer in self.answers.chunks_exact(WORD_LEN as usize) {
            for ((clue, &letter), guessed) in clues.iter_mut().zip(answer).zip(guess) {
                if guessed == letter {
                    *clue = Clue::Solved;
                } else {
                    unsolved_letters[(letter - b'a') as usize] += 1;
                }
            }
        }

        for (clue, guessed) in clues.iter_mut().zip(guess) {
            if *clue == Clue::Solved {
                continue;
            }
            if unsolved_letters[(guessed - b'a') as usize] > 0 {
                *clue = Clue::Misput;
                unsolved_letters[(guessed - b'a') as usize] -= 1;
            }
        }
        clues.into()
    }
}

// Two words remain
// that's not in the word list
// You got {answer}, one more to go.
// impl std::fmt::Display for Answers<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clue {
    Solved = 0,
    Misput = 1,
    Absent = 2,
}

#[repr(transparent)]
pub struct Outcome(Option<Clues>);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Clues([Clue; WORD_LEN as usize]);
impl Clues {
    pub fn iter(&self) -> std::slice::Iter<'_, Clue> {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a Clues {
    type Item = &'a Clue;
    type IntoIter = std::slice::Iter<'a, Clue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Outcome {
    pub fn clues(&self) -> Clues {
        match self {
            Outcome(None) => Clues([Clue::Absent; WORD_LEN as usize]),
            Outcome(Some(clues)) => *clues,
        }
    }
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Outcome(None) => write!(f, "🟩🟩🟩🟩🟩 answer found 🎆")?,
            Outcome(Some(clues)) => write!(f, "{clues}")?,
        }
        Ok(())
    }
}

impl std::fmt::Display for Clues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for clue in self.0 {
            match clue {
                Clue::Absent => write!(f, "⬛")?,
                Clue::Misput => write!(f, "🟨")?,
                Clue::Solved => write!(f, "🟩")?,
            }
        }
        Ok(())
    }
}

impl From<[Clue; WORD_LEN as usize]> for Outcome {
    fn from(value: [Clue; WORD_LEN as usize]) -> Self {
        Self(Some(Clues(value)))
    }
}
