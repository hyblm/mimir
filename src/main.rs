use mimir::solver::Solver;
use mimir::xordle::GameInstance;

fn main() {
    let mut game = GameInstance::new(["swamp", "teeth"]);
    let mut guesser = Solver::new();

    game.play(&mut guesser, true);
}
