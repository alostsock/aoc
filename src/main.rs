use clap::Parser;

/// Solves Advent of Code 2023 problems
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    /// A specific day of the month (1 to 25)
    #[arg(long)]
    day: Option<u8>,

    /// The part of a solution to run (1 or 2)
    #[arg(long)]
    part: Option<u8>,
}

fn main() {
    let args = Args::parse();

    if let Some(day) = args.day {
        if day > 25 {
            panic!("argument 'day' should be between 1 and 25");
        }
    }

    if let Some(part) = args.part {
        if part != 1 && part != 2 {
            panic!("argument 'part' should be 1 or 2");
        }
    }

    if args.day.is_some() {
        aoc::solve(args.day, args.part);
    } else {
        aoc::solve(None, None);
    }
}
