const LIFE_SPAN: usize = 8;

fn main() {

    let days = 256; //Change to 80 for part 1, to lazy to implement args

    let mut input = include_str!("input.txt").lines();

    // Lanternfish
    let mut lfs = vec![0; LIFE_SPAN + 1];

    let fish_timers = input.next().unwrap().split(',');
    for ft in fish_timers {
        let ft_value = ft.parse::<usize>().unwrap();
        lfs[ft_value] += 1;
    }

    for _ in 0..days {
        let parents = lfs[0];
        
        for i in 0..LIFE_SPAN as usize {
            lfs[i] = lfs[i + 1];
        }
        
        lfs[6] += parents;
        lfs[8] = parents;
    }

    let count: i64 = lfs.iter().sum();

    println!("Number of fishes at day {} is {}", days, count);
}