use std::{
    convert, env,
    fmt::Display,
    fs,
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

pub fn run_with_parser<I, I1, I2, R1, R2>(
    parse: impl FnOnce(String) -> I,
    part1: impl FnOnce(&I1) -> R1,
    part2: impl FnOnce(&I2) -> R2,
) where
    I: AsRef<I1> + AsRef<I2>,
    I1: ?Sized,
    I2: ?Sized,
    R1: Display,
    R2: Display,
{
    let input = parse(input());
    println!("part 1: {}", part1(input.as_ref()));
    println!("part 2: {}", part2(input.as_ref()));
}

pub fn run<I1, I2, R1, R2>(part1: impl FnOnce(&I1) -> R1, part2: impl FnOnce(&I2) -> R2)
where
    String: AsRef<I1> + AsRef<I2>,
    I1: ?Sized,
    I2: ?Sized,
    R1: Display,
    R2: Display,
{
    run_with_parser(convert::identity::<String>, part1, part2);
}
