use mimir::Game;

fn main() {
    let game = Game::new(&["swamp", "teeth"]);
    game.print_answers();
    println!("s e v e n");
    let rating = game.rate_guess(&['s', 'e', 'v', 'e', 'n']);
    println!("{}", rating);

    println!("f a u n a");
    let rating = game.rate_guess(&['f', 'a', 'u', 'n', 'a']);
    println!("{}", rating);

    println!("p o w e r");
    let rating = game.rate_guess(&['p', 'o', 'w', 'e', 'r']);
    println!("{}", rating);
}
