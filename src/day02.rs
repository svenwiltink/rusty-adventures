use std::error::Error;
use std::fs;

pub fn run(input: &str) -> Result<(), Box<dyn Error>> {
    let input = parse(input)?;
    let valid= input.iter().filter(|l|l.is_valid()).count();
    println!("valid {valid}");
    let valid= input.iter().filter(|l|l.is_valid_mutations()).count();
    println!("valid {valid}");
    Ok(())
}

fn parse(input: &str) -> Result<Input, Box<dyn Error>> {
    let input = fs::read_to_string(input)?;

    let input: Input = input.lines().map(|line| {
        Level::from_str(line)
    }).collect();


    Ok(input)
}

type Input = Vec<Level>;

#[derive(Debug)]
struct Level {
    data: Vec<i32>
}

impl Level {
    fn from_str(line: &str) -> Self {
        Self {
            data: line.split(" ").map(|n| n.parse()).flatten().collect()
        }
    }

    fn is_valid(&self) -> bool {

        let diffs: Vec<i32> = self.data
            .windows(2)
            .map(|c| c[0] - c[1])
            .collect();

        let increasing = diffs.iter()
            .map(|item| *item > 0)
            .all(|x| x == true);

        let decreasing = diffs.iter()
            .map(|item| *item < 0)
            .all(|x| x == true);

        if !increasing && !decreasing {
            return false
        }

        diffs.iter()
            .map(|x| x.abs())
            .map(|x| x > 0 && x <= 3)
            .all(|x| x == true)
    }

    fn is_valid_mutations(&self) -> bool{
        if self.is_valid() {
            return true
        }

        for i in 0..self.data.len(){
            let mut copy = self.data.clone();
            copy.remove(i);
            let level = Level{data: copy};
            if level.is_valid() {
                return true
            }
        }
        false
    }
}
