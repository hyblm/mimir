use crate::{Outcome, Solver};

pub const WORD_LEN: usize = 5;
const WORD_COUNT: usize = 2;
const ANSWERS_LEN: usize = WORD_COUNT * WORD_LEN;

pub type Word = [u8; WORD_LEN];
pub type Mark = [Clue; WORD_LEN];

pub struct Game {
    word_buffer: [u8; 3 * WORD_LEN],
    history: Vec<(Word, Outcome)>,
}

pub const MAX_ATTEMPTS: usize = 25;
pub const RAN_OUT_OF_ATTEMPTS: usize = 0;
pub const FAILED_TO_GUESS: usize = 1;

pub static ALL_GAMES: &str = include_str!("../word_lists/answers_with_first_guess.txt");
impl Game {
    pub fn play_with(&mut self, solver: &mut impl Solver) -> usize {
        let mut found_answer_count = 0;
        for i in 1..=MAX_ATTEMPTS {
            if found_answer_count >= WORD_COUNT {
                return i;
            }

            let Some(guess) = solver.make_guess() else {
                return FAILED_TO_GUESS;
            };
            let outcome = self.mark_guess(guess);
            if outcome.0 {
                found_answer_count += 1
            };
            solver.judge_outcome(guess, outcome);
        }

        RAN_OUT_OF_ATTEMPTS
    }

    pub fn from_archive(idx: usize) -> Self {
        let start = idx * (WORD_LEN * 3 + 1);
        let end = start + WORD_LEN * 3;
        let str = &ALL_GAMES[start..end];
        Self::from_str(str)
    }
    pub fn from_str(str: &str) -> Self {
        let mut this = Self {
            word_buffer: str.as_bytes().try_into().unwrap(),
            history: Vec::new(),
        };

        this.mark_guess(*this.get_word(2));
        this
    }

    fn get_word(&self, index: usize) -> &Word {
        let start = index * WORD_LEN;
        let end = start + WORD_LEN;
        self.word_buffer[start..end].try_into().unwrap()
    }

    fn get_word_as_str(&self, index: usize) -> &str {
        str::from_utf8(self.get_word(index)).unwrap()
    }

    fn mark_guess(&mut self, guess: Word) -> (bool, Mark) {
        let outcome = if guess == *self.get_word(0) || guess == *self.get_word(1) {
            (true, [Clue::Solved; 5])
        } else {
            // a LUT of how many times a given letter was contained in the answer but not in guess
            let mut unsolved_letters = [0u8; (b'z' - b'a' + 1) as usize];

            for &letter in self.word_buffer[..ANSWERS_LEN].into_iter() {
                unsolved_letters[(letter - b'a') as usize] += 1;
            }

            let answer1 = self.get_word(0);
            let answer2 = self.get_word(1);
            let mut clues = [Clue::Absent; WORD_LEN as usize];

            for i in 0..WORD_LEN {
                if guess[i] == answer1[i] || guess[i] == answer2[i] {
                    clues[i] = Clue::Solved;
                } else if unsolved_letters[(guess[i] - b'a') as usize] > 0 {
                    clues[i] = Clue::Misput;
                    unsolved_letters[(guess[i] - b'a') as usize] -= 1;
                }
            }
            (false, clues)
        };

        self.history.push((guess, outcome));
        outcome
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "answers: [{}, {}]",
            self.get_word_as_str(0),
            self.get_word_as_str(1),
        )?;
        writeln!(f, "----------")?;

        for (guess, (was_correct, mark)) in &self.history {
            for letter in str::from_utf8(guess).unwrap().to_uppercase().chars() {
                write!(f, "{letter} ")?
            }
            writeln!(f)?;
            if *was_correct {
                writeln!(f, "🌠🌠🌠🌠🌠")?;
            } else {
                for clue in mark {
                    match clue {
                        Clue::Absent => write!(f, "⬛")?,
                        Clue::Misput => write!(f, "🟨")?,
                        Clue::Solved => write!(f, "🟩")?,
                    }
                }
                writeln!(f)?;
            }
        }
        writeln!(f)
    }
}

// static ALPHABET: &str = "
// 🄰 🄱 🄲 🄳 🄴 🄵 🄶 🄷 🄸 🄹 🄺 🄻 🄼 🄽 🄾 🄿 🅀 🅁 🅂 🅃 🅄 🅅 🅆 🅇 🅈 🅉
// 🅐 🅑 🅒 🅓 🅔 🅕 🅖 🅗 🅘 🅙 🅚 🅛 🅜 🅝 🅞 🅟 🅠 🅡 🅢 🅣 🅤 🅥 🅦 🅧 🅨 🅩
// ";

// 🟩🟥🟨⬜⬛🟪
// 🄳 🄸 🅃 🅃 🄾

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clue {
    Solved = 0,
    Misput = 1,
    Absent = 2,
}
