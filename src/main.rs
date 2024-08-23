use mimir::{
    xordle::{word, Answers, ANSWERS_LEN, WORD_LEN},
    Solver, DISPLAY, MAX_ATTEMPTS,
};

fn main() {
    play_all_games_with_first_guess();
}

const ANSWERS_WITH_FIRST_GUESS: &str = include_str!("../word_lists/answers_with_first_guess.txt");

pub fn play_all_games_with_first_guess() {
    let mut stats = Stats::new();
    for (round, line) in ANSWERS_WITH_FIRST_GUESS.lines().enumerate() {
        let (answers, initial_guess) = line.split_at(ANSWERS_LEN as usize);
        if DISPLAY {
            print!("{}{}", console::RESET_CURSOR, console::CLEAR);
            println!(
                "day {:4} / {}  -  [ {}, {} ]",
                round + 1,
                stats.rounds_total,
                &answers[..WORD_LEN as usize],
                &answers[WORD_LEN as usize..]
            );
        }
        let mut answers = Answers::new(answers.as_bytes());
        let guess = word(initial_guess.as_bytes());
        let outcome = answers.compare_guess(guess);
        if DISPLAY {
            print!("{initial_guess} {outcome}");
        }
        let mut mimir = mimir::solver();
        mimir.judge_outcome(guess, outcome, &answers);
        let score = mimir.play(&mut answers).expect("failed to guess");
        stats.rounds_solved_in[score as usize] += 1;
    }
    stats.print();
}

struct Stats {
    rounds_total: usize,
    // it is impossible to guess the answer in 0 or 1 attempts,
    // so we'll use those spots for counting games that ended with:
    // [0] => the solver failing to produce a guess
    // [1] => the solver running out of attempts
    rounds_solved_in: [u8; MAX_ATTEMPTS as usize],
}

impl Stats {
    pub fn new() -> Self {
        let rounds_total = ANSWERS_WITH_FIRST_GUESS.len() / (ANSWERS_LEN + WORD_LEN + 1) as usize;
        Self {
            rounds_total,
            rounds_solved_in: [0u8; MAX_ATTEMPTS as usize],
        }
    }

    pub fn print(&self) {
        let failed_games = (self.rounds_solved_in[0] + self.rounds_solved_in[1]) as usize;
        println!();
        // println!("Couldn't make guess {} times", self.rounds_solved_in[0]);
        // println!("Ran out of attempts {} times", self.rounds_solved_in[1]);
        let mut solved_games = 0;
        let mut sum = 0f64;
        for (attempts, count) in self.rounds_solved_in.iter().enumerate().skip(2) {
            solved_games += usize::from(*count);
            sum += (attempts * *count as usize) as f64;
            println!("{attempts:2} {count}");
            if solved_games + failed_games == self.rounds_total {
                break;
            }
        }
        println!(
            "played {solved_games} games with the average Score of {}",
            // self.rounds_total,
            sum / solved_games as f64
        );
    }
}

mod console {
    pub const CLEAR: &str = "\x1B[J";
    pub const RESET_CURSOR: &str = "\x1B[0;0H";
}
