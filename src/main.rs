use mimir::{Stats, xordle};

fn main() {
    let mut stats = Stats::new();
    for i in 0..1_000 {
        let mut game = xordle::Game::from_archive(i);
        let mut simple_solver = mimir::solver::Simple::new();
        let score = game.play_with(&mut simple_solver);
        stats.bump(score);
    }

    stats.print_histagram();
}
