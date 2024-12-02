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
    let lines = input.lines().map(|x| {
        x.split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });

    Ok(lines
        .filter(|line| {
            let mut line = line.iter();

            let mut previous = *line.next().unwrap();
            let mut current = *line.next().unwrap();

            let upper_bound: i32;
            let lower_bound: i32;
            if previous > current {
                upper_bound = 3;
                lower_bound = 1;
            } else if previous < current {
                upper_bound = -1;
                lower_bound = -3;
            } else {
                return false;
            }

            loop {
                let difference = previous - current;
                if difference < lower_bound || difference > upper_bound {
                    return false;
                }

                previous = current;

                if let Some(next) = line.next() {
                    current = *next;
                } else {
                    return true;
                }
            }
        })
        .count() as i32)
}
