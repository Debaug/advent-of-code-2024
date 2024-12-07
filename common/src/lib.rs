use std::{
    env, fs,
    io::{self, Read},
};

pub fn input() -> String {
    match env::args().nth(1) {
        Some(file) => fs::read_to_string(file).expect("failed to read input file"),
        None => {
            eprintln!("reading from stdin...");
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .expect("failed to read from stdin");
            input
        }
    }
}
