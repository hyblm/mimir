use mimir::xordle;

fn main() {
    let mut stats = Stats::new();
    for i in 0..1_000 {
        let mut game = xordle::Game::from_archive(i);
        let mut simple_solver = mimir::Simple::new();
        let score = game.play_with(&mut simple_solver);
        stats.bump(score);
        // println!("{game}");
    }

    stats.print_histagram();
}

#[derive(Default, Debug)]
struct Stats {
    table: [usize; xordle::MAX_ATTEMPTS],
}

impl Stats {
    fn new() -> Self {
        Self::default()
    }

    fn bump(&mut self, idx: usize) {
        self.table[idx] += 1;
    }

    fn print_histagram(&self) {
        let total: usize = self.table.iter().sum();
        let max_count = self.table.iter().copied().max().unwrap_or(0);

        println!("Score histogram ({total} games):");
        for (score, count) in self.table.iter().copied().enumerate() {
            let label = match score {
                xordle::RAN_OUT_OF_ATTEMPTS => "ran out".to_string(),
                xordle::FAILED_TO_GUESS => "failed".to_string(),
                attempts => format!("{attempts} attempts"),
            };
            let bar_len = if max_count == 0 {
                0
            } else {
                count * 50 / max_count
            };
            let bar = "█".repeat(bar_len);

            println!("{label:>12} ┃ {count:>5} {bar}");
        }
    }
}
