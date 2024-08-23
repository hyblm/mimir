// use std::io;
//
// use crate::{xordle::Word, Feedback, WORD_LEN};
//
// mod dictionary;
// use dictionary::Dictionary;

// pub struct Solver {
//     remaining: Dictionary,
// }

// impl Solver {
//     pub fn new() -> Self {
//         Self {
//             remaining: Dictionary::new(),
//         }
//     }
//
//     pub fn make_guess(&mut self) -> Word {
//         // let total_count: u64 = self.remaining.counts.iter().sum();
//         //
//         // for &count in &self.remaining.counts {
//         //     let probability = count as f64 / total_count as f64;
//         // }
//         let index = 0;
//         let _ = self.remaining.counts.remove(index);
//         let guess = self.remaining.words.remove(index);
//         guess
//     }
//
//     pub fn type_guess() -> Word {
//         let mut input_buffer = String::new();
//
//         let stdin = io::stdin(); // We get `Stdin` here.
//         loop {
//             if let Ok(n) = stdin.read_line(&mut input_buffer) {
//                 if n == WORD_LEN + 1 {
//                     if input_buffer.is_ascii() {
//                         let mut guess = [0; WORD_LEN];
//                         for (letter, byte) in guess.iter_mut().zip(input_buffer.bytes()) {
//                             *letter = byte;
//                         }
//                         return guess;
//                     } else {
//                         println!("Guess contains non ASCII characters");
//                     }
//                 } else {
//                     println!("Guess needs to be {} letters", WORD_LEN);
//                 }
//             }
//         }
//     }
//
//     // NOTE: both grey and yellow give us information about both target words
//     // - grey tells us that the letter isn't present at all in either answer
//     // - yellow tells us that the letter isn't in the guessed position in either answer
//     //
//     // We can trim
//     // - all words that contain Absent letters
//     // - all words that contain Misput letters in the location where they was guessed
//     pub fn register_feedback(&mut self, guess: Word, feedback: Feedback) {
//         self.remaining.trim(guess, feedback);
//     }
// }
