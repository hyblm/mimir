use mimir::Game;

fn main() {
    let game = Game::new(&["swamp", "teeth"]);
    println!("{}", game);
    println!("s e v e n");
    let rating = game.rate_guess(b"seven");
    println!("{}", rating);

    println!("f a u n a");
    let rating = game.rate_guess(b"fauna");
    println!("{}", rating);

    println!("p o w e r");
    let rating = game.rate_guess(b"power");
    println!("{}", rating);

    let game = Game::new(&["funds", "voter"]);
    println!("{}", game);
    println!("p r o x y");
    let rating = game.rate_guess(b"proxy");
    println!("{}", rating);

    println!("s c a l d");
    let rating = game.rate_guess(b"scald");
    println!("{}", rating);

    println!("q u i t e");
    let rating = game.rate_guess(b"quite");
    println!("{}", rating);

    println!("m u t e s");
    let rating = game.rate_guess(b"mutes");
    println!("{}", rating);

    println!("d o n o r");
    let rating = game.rate_guess(b"donor");
    println!("{}", rating);

    println!("b o n e s");
    let rating = game.rate_guess(b"bones");
    println!("{}", rating);

    println!("f u n d s");
    let rating = game.rate_guess(b"funds");
    println!("{}", rating);

    println!("t o t e r");
    let rating = game.rate_guess(b"toter");
    println!("{}", rating);

    println!("v o t e r");
    let rating = game.rate_guess(b"voter");
    println!("{}", rating);
}
