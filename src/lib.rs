const WORD_LEN: usize = 5;
const NUM_WORDS: usize = 2;
const ANSWERS_LEN: usize = NUM_WORDS * WORD_LEN;
// const ANSWERS: &str = include_str!("../answers.txt");
// const DICTIONARY: &str = include_str!("../dictionary.txt");

pub struct Game {
    answers: [char; ANSWERS_LEN],
}

type Word = [char; WORD_LEN];
impl Game {
    pub fn new(words: &[&str]) -> Self {
        let mut answers = [' '; ANSWERS_LEN];
        for (i, letter) in words.concat().chars().enumerate() {
            answers[i] = letter;
        }

        Self { answers }
    }

    pub fn print_answers(&self) {
        print!("\n Answers: ");
        for word in self.answers.chunks(WORD_LEN) {
            for letter in word {
                print!("{}", letter);
            }
            print!(" ")
        }
        print!("\n")
    }

    pub fn rate_guess(&self, guess: &Word) -> Rating {
        assert_eq!(guess.len(), WORD_LEN);

        use Color::*;
        let mut mask = [Gray; WORD_LEN];
        let mut used_letters = [false; ANSWERS_LEN];

        // Check if Green
        for (i, (letter, color)) in guess.into_iter().zip(&mut mask).enumerate() {
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
        let turn_yellow = |letter: &char, answer: &[char], taken: &mut [bool]| -> bool {
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
        let rating = game.rate_guess(&['o', 'r', 'd', 'e', 'r']);
        for (e, r) in expected.into_iter().zip(rating.mask) {
            assert_eq!(e, r);
        }
    }
}
