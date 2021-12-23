use std::{fs::File, io::{BufReader, BufRead}};

fn main() {

    let input = parse_input("inputs/d01.txt");

    let mut increases = 0;

    if input.len() > 1 {
        let mut prev = input[0];

        for i in input.iter().skip(1) {
            if *i > prev {
                increases += 1;
            }
            prev = *i;
        }
    }

    println!("Number of increases: {}", increases);

    let mut sum_increases = 0;
    if input.len() > 4 {
        let mut sum = input[0] + input[1] + input[2];

        for (pos, i ) in input.iter().enumerate().skip(3) {
            let prev = sum;
            sum += *i - input[pos-3];
            if sum > prev {
                sum_increases += 1;
            }
        }
    }
    
    println!("Number of sum_increases: {}", sum_increases);
}

pub fn parse_input(path: &str) -> Vec<i32> {
    let file = File::open(path).expect("Something went wrong");
    let reader = BufReader::new(file);

    let mut res: Vec<i32> = Vec::new();

    for line_res in reader.lines() {
        if let Ok(line) = line_res {
            if let Ok(t) = line.parse::<i32>() {
                res.push(t);
            }
        }
    }

    res
}
