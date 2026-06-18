pub mod xordle;

pub mod console {
    #![allow(unused)]

    pub const CLEAR: &str = "\x1B[J";
    pub const RESET_CURSOR: &str = "\x1B[0;0H";
    pub const GREEN_BG: &str = "\x1B[102m";
    pub const RED_BG: &str = "\x1B[101m";
    pub const BOLD: &str = "\x1B[1m";
    pub const YELLOW_BG: &str = "\x1B[103m";
    pub const BLACK_FG: &str = "\x1B[30m";
    pub const RESET_COLORS: &str = "\x1B[0m";
}
// #[cfg(test)]
// mod tests;

pub const DISPLAY: bool = false;
pub const MAX_ATTEMPTS: u8 = 50;

pub use solver::Solver;
pub mod solver;

#[derive(Default, Debug)]
pub struct Stats {
    table: [usize; xordle::MAX_ATTEMPTS],
}

impl Stats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bump(&mut self, idx: usize) {
        self.table[idx] += 1;
    }

    pub fn print_histagram(&self) {
        let total: usize = self.table.iter().sum();
        let max_count = self.table.iter().copied().max().unwrap_or(0);

        println!("Score histogram ({total} games):");
        for (score, count) in self.table.iter().copied().enumerate() {
            let label = match score {
                xordle::RAN_OUT_OF_ATTEMPTS => "ran out".to_string(),
                xordle::FAILED_TO_GUESS => "failed".to_string(),
                attempts => format!("{attempts} attempts"),
            };
            let bar = Self::histagram_bar(count, max_count, 50);

            println!("{label:>12} ┃ {count:>5} {bar}");
        }
    }

    fn histagram_bar(count: usize, max_count: usize, width: usize) -> String {
        if max_count == 0 || count == 0 {
            return String::new();
        }

        let quarter_blocks = count * width * 4 / max_count;
        let full_blocks = quarter_blocks / 4;
        let partial_block = match quarter_blocks % 4 {
            0 => "",
            1 => "▎",
            2 => "▌",
            3 => "▊",
            _ => unreachable!(),
        };

        format!("{}{}", "█".repeat(full_blocks), partial_block)
    }
}
