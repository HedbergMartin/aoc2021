
// Should probobly not do this much error handling in the next days.
fn main() {
    let file = include_str!("input.txt");

    let res: (i32, i32, i32) = file.lines().map(|s| {
        s.split_once(" ")
    }).fold((0, 0, 0), |(x, y, aim), val: Option<(&str, &str)>| {
        if let Some((dir, val_str)) = val{
            match val_str.parse::<i32>() {
                Ok(val) => {
                    match dir {
                        "forward" => return (x + val, y + (val * aim), aim),
                        "down" => return (x, y, aim + val),
                        "up" => return (x, y, aim - val),
                        no_dir => println!("{} not a valid dir", no_dir),
                    }
                },
                Err(e) => println!("Error {}", e),
            };
        }

        (x, y, aim)
    });

    println!("Horizontal: {}, depth: {}, multiplied: {}", res.0, res.1, res.0 * res.1);
}