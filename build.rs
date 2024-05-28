use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let dict_path = Path::new(&env::var("OUT_DIR").unwrap()).join("dict.rs");
    let mut dict_wirter = BufWriter::new(File::create(&dict_path).unwrap());
    let dict: Vec<_> = include_str!("dictionary.txt").lines().collect();
    let dict = quickphf_codegen::build_set(&dict);
    write!(&mut dict_wirter, "{}", dict).unwrap();

    let answers_path = Path::new(&env::var("OUT_DIR").unwrap()).join("answers.rs");
    let mut answers_writer = BufWriter::new(File::create(&answers_path).unwrap());
    let answers: Vec<_> = include_str!("answers.txt").lines().collect();
    let answers = quickphf_codegen::build_set(&answers);
    write!(&mut answers_writer, "{}", answers).unwrap();
}
