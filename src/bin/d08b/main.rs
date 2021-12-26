
fn main() {
    let lines = include_str!("input.txt").lines();
    
    let entries: Vec<Entry> = lines.map(|x| Entry::new(x)).collect();
    
    let count: i32 = entries.iter().map(|x| x.decode_output()).sum();
    
    println!("Sum of outputs: {}", count);
}

struct Entry<'a> {
    output: Vec<&'a str>,
    coded: Vec<Option<&'a str>>,
}

impl<'l> Entry<'l> {
    fn new(s: &'l str) -> Self {
        let sides = s.split_once('|').unwrap();
        let input = sides.0.split_whitespace().collect::<Vec<&str>>();
        let output = sides.1.split_whitespace().collect::<Vec<&str>>();

        let coded = Entry::decode(&input, &output);

        Self {
            output,
            coded,
        }
    }

    fn decode(input: &Vec<&'l str>, output: &Vec<&'l str>) -> Vec<Option<&'l str>>{
        let mut results: Vec<Option<&str>> = vec![None; 9];
        
        let strings = input.iter().chain(output.iter());
        
        let mut base_cases = 0;
        for &s in strings {
            if base_cases >= 4 {
                break;
            }
            match s.len() {
                2 => base_cases += Entry::add_string(&mut results, 1, s),
                3 => base_cases += Entry::add_string(&mut results, 7, s),
                4 => base_cases += Entry::add_string(&mut results, 4, s),
                7 => base_cases += Entry::add_string(&mut results, 8, s),
                _ => (),
            }
        }

        if base_cases < 4 {
            println!("TOO FEW BASE CASES");
            panic!();
        }

        results
    }

    fn str_includes(s1: &str, s2: &str) -> bool {
        s1.chars().all(|x| {s2.contains(x)})
    }

    fn str_matches(s1: &str, s2: &str) -> usize {
        s1.chars().filter(|&x| {s2.contains(x)}).count()
    }

    fn decode_output(&self) -> i32 {
        let mut val = 0;
        for &s in self.output.iter() {
            val = val * 10 + match s.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                6 => { // 0, 6, 9
                    if Entry::str_includes(self.coded[1].unwrap(), s) {
                        if Entry::str_includes(self.coded[4].unwrap(), s) {
                            9
                        } else {
                            0
                        }
                    } else {
                        6
                    }
                },
                5 => { // 2, 3, 5
                    if Entry::str_includes(self.coded[1].unwrap(), s) {
                        3
                    } else {
                        let matches = Entry::str_matches(self.coded[4].unwrap(), s);
                        if matches == 2 {
                            2
                        } else {
                            5
                        }
                    }
                },
                _ => 0,
            }
        }

        val
    }

    fn add_string(results: &mut Vec<Option<&'l str>>, index: usize, s: &'l str) -> usize {
        if results[index].is_none() {
            results[index] = Some(s);
            1
        } else {
            0
        }
    }
}