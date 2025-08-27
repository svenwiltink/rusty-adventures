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
    data: Vec<u32>
}

impl Level {
    fn from_str(line: &str) -> Self {
        Self {
            data: line.split(" ").map(|n| n.parse()).flatten().collect()
        }
    }

    fn is_valid(&self) -> bool {
        let increasing = self.all_increasing();
        let decreasing = self.all_decreasing();

        if !increasing && !decreasing {
            return false
        }

        self.steps_valid()
    }

    fn steps_valid(&self) -> bool {
        let mut current = &self.data[0];

        for item in &self.data[1..] {
            if item.abs_diff(*current) > 3 {
                return false
            }

            current = item
        }

        true
    }

    fn all_increasing(&self) -> bool {
        let mut current = &self.data[0];

        for item in &self.data[1..] {
            if item <= current {
                return false;
            }

            current = item
        }

        true
    }

    fn all_decreasing(&self) -> bool {
        let mut current = &self.data[0];

        for item in &self.data[1..] {
            if item >= current {
                return false;
            }

            current = item
        }

        true
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
