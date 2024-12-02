use anyhow::{bail, Result};
use std::{env::args, fs::read_to_string};

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        bail!("Wrong amount of arguments, expected filename for input file.")
    }

    let filename = &args[1];
    let input = read_to_string(filename)?;

    println!("{}", solve(&input)?);

    Ok(())
}

fn solve(input: &str) -> Result<i32> {
    todo!()
}
