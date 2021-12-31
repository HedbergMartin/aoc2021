use std::str::Chars;

const OPEN: &str = "([{<";
const CLOSE: &str = ")]}>";

const ERROR_POINTS: [i32; 4] = [ 3, 57, 1197, 25137 ];
const COMPLETE_POINTS: [i64; 4] = [ 1, 2, 3, 4 ];

fn main() {
    let lines = include_str!("input.txt").lines();

    let mut points = 0; // Error score
    let mut incomplete_scores: Vec<i64> = Vec::new();
    for l in lines {
        let mut cs = l.chars();
        let res = search(&mut cs);
        if let Err(e) = res {
            points += e.0;
            if e.1 != 0 {
                incomplete_scores.push(e.1);
            }
        }
    }

    incomplete_scores.sort_unstable();

    println!("Syntax error score: {}, middle incomplete score: {}", points, incomplete_scores[incomplete_scores.len()/2]);
}

fn search(iter: &mut Chars) -> Result<(), (i32, i64)> {
    let open = iter.next().unwrap();
    search_rec(open, iter)
}

fn search_rec(open: char, iter: &mut Chars) -> Result<(), (i32, i64)> {
    let next = loop {
        if let Some(n) = iter.next() {
            if is_open(n) {
                if let Err((es, cs)) = search_rec(n, iter) {
                    let new_cs = if cs != 0 {
                        (cs * 5) + get_complete_point(open)
                    } else {
                        0
                    };

                    return Err((es, new_cs));
                }
            } else {
                break n;
            }
        } else {
            return Err((0, get_complete_point(open)));
        }
    };

    if matches(open, next) {
        Ok(())
    } else {
        Err((get_error_point(next), 0))
    }
}

fn is_open(c: char) -> bool {
    OPEN.contains(c)
}

fn get_error_point(c: char) -> i32 {
    ERROR_POINTS[CLOSE.chars().position(|ch| ch == c).unwrap()]
}

fn get_complete_point(c: char) -> i64 {
    COMPLETE_POINTS[OPEN.chars().position(|ch| ch == c).unwrap()]
}

fn matches(c_open: char, c_close: char) -> bool {
    OPEN.chars().position(|c| c == c_open).unwrap() == CLOSE.chars().position(|c| c == c_close).unwrap()
}
