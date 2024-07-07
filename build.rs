use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let output_directory = &env::var("OUT_DIR").unwrap();

    let (keys, values): (Vec<_>, Vec<_>) = include_str!("dictionary_counts.txt")
        .lines()
        .map(|line| {
            let (word, count) = line.split_once(' ').expect("line is in form `word count`");
            let count: usize = count.parse().expect("count is a whole number");
            (word, count)
        })
        .unzip();
    let dictionary = quickphf_codegen::build_map(&keys, &values);

    let dictionary_path = Path::new(output_directory).join("dict.rs");
    let mut writer = BufWriter::new(File::create(dictionary_path).unwrap());
    write!(&mut writer, "{}", dictionary).unwrap();
}
