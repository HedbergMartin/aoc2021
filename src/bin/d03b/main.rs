const BITS: usize = 12;

fn main() {
    let lines: Vec<&str> = include_str!("input.txt").lines().collect();

    let mut og_ratings = lines.clone();
    for i in 0..BITS {
        og_ratings = filter_lines(og_ratings, i, |x| x >= 0);
        if og_ratings.len() == 1 {
            break;
        }
    }

    let og_rating = i32::from_str_radix(og_ratings[0], 2).unwrap();

    let mut cs_ratings = lines.clone();
    for i in 0..BITS {
        cs_ratings = filter_lines(cs_ratings, i, |x| x < 0);
        if cs_ratings.len() == 1 {
            break;
        }
    }

    let cs_rating = i32::from_str_radix(cs_ratings[0], 2).unwrap();

    println!("OG_rating = {}, CS_rating = {}", og_rating, cs_rating);
    println!("LS_rating = {}", og_rating * cs_rating);
}

fn filter_lines<Comp: Fn(i32) -> bool>(lines: Vec<&str>, index: usize, compare: Comp) -> Vec<&str> {
    let mut counter = 0;
    lines.iter().for_each(|line| {
        let c = line.chars().nth(index).unwrap();
        match c {
            '1' => counter += 1,
            '0' => counter -= 1,
            _ => (),
        }
    });

    let compare_value = compare(counter);

    lines.iter().copied().filter(|&line| {
        let c = line.chars().nth(index).unwrap();
        match c {
            '1' => compare_value,
            '0' => !compare_value,
            _ => false,
        }
    }).collect()
}
