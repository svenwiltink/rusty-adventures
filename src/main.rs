use std::env;
use std::error::Error;
use std::process::exit;

mod day01;
mod day02;

fn main() -> Result<(), Box<dyn Error>>{
    let  args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("two params required: day and input file location");
        exit(1);
    }

    let day = args[1]
        .parse()
        .or_else(|e| Err(format!("error parsing day: {e}")))?;
    let file = &args[2];

    match day {
        1 => day01::run(file),
        2 => day02::run(file),
        d => Err(format!("day {d} not implemented").into())
    }
}