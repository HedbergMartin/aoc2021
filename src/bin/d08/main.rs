

fn main() {
    let lines = include_str!("input.txt").lines();

    let entries: Vec<Entry> = lines.map(|x| Entry::new(x)).collect();

    let count: i32 = entries.iter().map(|x| x.count_occurence_output().iter().sum::<i32>()).sum();

    println!("Unique digit occurence: {}", count);
}

#[allow(dead_code)]
struct Entry<'a> {
    input: Vec<&'a str>,
    output: Vec<&'a str>,
}

impl<'l> Entry<'l> {
    fn new(s: &'l str) -> Self {
        let sides = s.split_once('|').unwrap();
        let input = sides.0.split_whitespace().collect::<Vec<&str>>();
        let output = sides.1.split_whitespace().collect::<Vec<&str>>();

        Self {
            input,
            output,
        }
    }

    fn count_occurence_output(&self) -> Vec<i32> {
        let mut occur = vec![0; 10];

        for &o in self.output.iter() {
            match o.len() {
                2 => occur[1] += 1,
                3 => occur[7] += 1,
                4 => occur[4] += 1,
                7 => occur[8] += 1,
                _ => (),
            }
        }

        occur
    }
}