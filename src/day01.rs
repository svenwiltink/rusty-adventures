use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(input: &str) -> Result<(), Box<dyn Error>>{
    let (mut left, mut right) = parse(&input)?;

    left.sort();
    right.sort();

    let mut distance = 0;
    for i in 0..left.len() {
        distance += left[i].abs_diff(right[i])
    }

    println!("distance: {distance}");

    let mut sim = 0;
    for i in left {
        let amount = count(i, &right);
        sim += i * amount
    }

    println!("similarity: {sim}");
    Ok(())
}

fn count(needle: usize, haystack: &Vec<usize>) -> usize {
    let mut res = 0;
    for item in haystack {
        if needle == *item {
            res += 1;
        }
    }

    res
}

fn parse(input: &str) -> Result<(Vec<usize>, Vec<usize>), Box<dyn Error>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);


    let mut left = vec![];
    let mut right = vec![];
    for line in reader.lines() {
        let l = line.or_else(|err| Err(format!("error parsing line: {err}")))?;
        let mut words = l.split_whitespace();
        left.push(words.next().ok_or("left num missing")?.parse()?);
        right.push(words.next().ok_or("right num missing")?.parse()?);
    }

    Ok((left,right))
}
