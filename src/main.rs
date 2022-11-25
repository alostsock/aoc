use clap::Parser;

/// A program to run solutions for Advent of Code in 2022
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    /// A specific day of the month (1 to 25)
    #[arg(long)]
    day: u8,

    /// The part of a solution to run (1 or 2)
    #[arg(long)]
    part: Option<u8>,
}

fn main() {
    let args = Args::parse();

    if args.day > 25 {
        panic!("argument 'day' should be between 1 and 25");
    }

    if let Some(part) = args.part {
        if part != 1 && part != 2 {
            panic!("argument 'part' should be 1 or 2");
        }
    }

    let part_text = if args.part.is_some() {
        format!(", part {}", args.part.unwrap())
    } else {
        "".to_string()
    };
    println!("\nRunning day {}{}...", args.day, part_text);

    aoc::run_solution(args.day, args.part);
}
