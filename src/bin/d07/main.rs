

fn main() {
    let mut input = include_str!("input.txt").lines();

    let positions: Vec<i32> = input.next().unwrap().split(',').map(|s| {
        s.parse::<i32>().unwrap()
    }).collect();

    // Probobly inefficient but w/e
    let max = *positions.iter().max().unwrap();
    let min = *positions.iter().min().unwrap();

    let mut move_costs: Vec<i32> = Vec::with_capacity((max-min) as usize);
    for i in min..max {
        move_costs.push(calculate_move_cost(&positions, i));
    }

    let mut cheapest_move = (0 as usize, move_costs[0]);

    //Skip first
    for (i, val) in move_costs.iter().enumerate().skip(1) {
        if *val < cheapest_move.1 {
            cheapest_move = (i, *val);
        }
    }

    println!("Moving all subs to {} would give the fuel consumption of {}, which is the lowest", cheapest_move.0, cheapest_move.1);
}

fn calculate_move_cost(pos: &Vec<i32>, to: i32) -> i32 {
    pos.iter().fold(0, |acc, &x| {
        acc + (to - x).abs()
    })
}