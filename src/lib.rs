use core::fmt;

const WORD_LEN: usize = 5;
const NUM_WORDS: usize = 2;
const ANSWERS_LEN: usize = NUM_WORDS * WORD_LEN;

static DICTIONARY: quickphf::PhfSet<&str> = include!(concat!(env!("OUT_DIR"), "/dict.rs"));
static ANSWERS: quickphf::PhfSet<&str> = include!(concat!(env!("OUT_DIR"), "/answers.rs"));

// NOTE (matyas): hello
pub struct Game {
    answers: [u8; ANSWERS_LEN],
}

type Word = [u8; WORD_LEN];
impl Game {
    pub fn new(answer_words: &[&str]) -> Self {
        let mut answers = [0; ANSWERS_LEN];
        for (i, letter) in answer_words.concat().bytes().enumerate() {
            answers[i] = letter;
        }

        Self { answers }
    }

    pub fn check(&self, guess: &'static Word) -> bool {
        DICTIONARY.contains(&std::str::from_utf8(guess).unwrap())
    }

    pub fn rate_guess(&self, guess: &Word) -> Rating {
        assert_eq!(guess.len(), WORD_LEN);

        use Color::*;
        let mut mask = [Gray; WORD_LEN];
        let mut used_letters = [false; ANSWERS_LEN];

        // Check if Green
        for (i, (letter, color)) in guess.iter().zip(&mut mask).enumerate() {
            for (answer, used) in self
                .answers
                .chunks(WORD_LEN)
                .zip(used_letters.chunks_mut(WORD_LEN))
            {
                used[i] = *letter == answer[i];
            }

            let correct_in_some_answer = used_letters.iter().skip(i).step_by(WORD_LEN).any(|&t| t);
            if correct_in_some_answer {
                *color = Green;
            }
        }

        // Check if yellow
        let turn_yellow = |letter: &u8, answer: &[u8], taken: &mut [bool]| -> bool {
            let answer = answer
                .iter()
                .zip(taken.as_ref())
                .position(|(answer, &taken)| !taken && letter == answer);
            if let Some(position) = answer {
                taken[position] = true;
            }
            answer.is_some()
        };
        let mut present_in_some_answer = |letter| {
            self.answers
                .chunks(WORD_LEN)
                .zip(used_letters.chunks_mut(WORD_LEN))
                .map(|(answer, taken)| turn_yellow(letter, answer, taken))
                .any(|t| t)
        };

        for (letter, color) in guess.iter().zip(&mut mask) {
            if present_in_some_answer(letter) {
                *color = Yellow;
            };
        }

        Rating { mask }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::with_capacity(self.answers.len() * WORD_LEN);
        out.push_str("\n Answers: ");
        for word in self.answers.chunks(WORD_LEN) {
            for &letter in word {
                out.push_str(&format!("{}", letter as char));
            }
            out.push(' ');
        }
        write!(f, "{}", out)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Gray,
    Yellow,
    Green,
}

pub struct Rating {
    mask: [Color; WORD_LEN],
}

impl std::fmt::Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let colors = self
            .mask
            .map(|color| match color {
                Color::Gray => "⬛",
                Color::Yellow => "🟨",
                Color::Green => "🟩",
            })
            .concat();
        write!(f, "{}", colors)
    }
}

mod tests {

    #[test]
    fn test2() {
        use crate::{Color::*, Game};
        let expected = [Yellow, Green, Gray, Gray, Gray];
        let game = Game::new(&["small", "group"]);
        let rating = game.rate_guess(b"order");
        for (e, r) in expected.into_iter().zip(rating.mask) {
            assert_eq!(e, r);
        }
    }
}
