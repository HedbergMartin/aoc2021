
fn main() {
    let lines = include_str!("input.txt").lines();

    let mut hm = HeightMap::new();

    for l in lines {
        hm.expand(l);
    }

    println!("Low point sum {}", hm.sum_lowpoints());
    println!("3 biggest basin sums {}", hm.sum_basins());
}

struct HeightMap {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl HeightMap {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            height: 0,
            width: 0,
        }
    }

    fn expand(&mut self, s: &str) {
        if self.width == 0 {
            self.width = s.len();
        }

        let mut digits: Vec<u8> = s.chars().map(|c| {
            c.to_digit(10).unwrap() as u8
        }).collect();

        self.data.append(&mut digits);
        self.height += 1;
    }

    fn sum_lowpoints(&self) -> u32 {
        let mut sum: u32 = 0;
        for i in 0..self.data.len() {
            sum += self.get_lowpoint_value(i) as u32;
        }

        sum
    }

    fn sum_basins(&self) -> i32 {
        let (basin_map, basins) = self.get_basin_map();
        let mut basin_sizes = vec![0; basins];

        for b in basin_map {
            if b != 0 {
                basin_sizes[b-1] += 1;
            }
        }

        basin_sizes.sort_unstable();
        basin_sizes.reverse();
        
        basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
    }

    fn get_basin_map(&self) -> (Vec<usize>, usize) {
        let mut map = vec![0 as usize; self.data.len()];

        let mut basin_id = 1;
        for i in 0..self.data.len() {
            if map[i] != 0 || self.data[i] == 9 {
                continue;
            }

            self.build_basin(i, &mut map, basin_id);
            basin_id += 1;
        }

        (map, basin_id-1)
    }

    fn build_basin(&self, index: usize, map: &mut Vec<usize>, basin_id: usize) {
        if map[index] != 0 || self.data[index] == 9 {
            return;
        }

        map[index] = basin_id;
        if index > self.width {
            self.build_basin(index - self.width, map, basin_id);
        }

        if (index+1) % self.width != 0 {
            self.build_basin(index + 1, map, basin_id);
        }

        if (index / self.width) + 1 < self.height {
            self.build_basin(index + self.width, map, basin_id);
        }

        if index % self.width != 0 {
            self.build_basin(index - 1, map, basin_id);
        }
    }

    fn get_lowpoint_value(&self, index: usize) -> u8{
        let north = match index < self.width {
            true => u8::MAX, //Top row
            false => self.data[index - self.width],
        };
        let east = match (index+1) % self.width == 0 {
            true => u8::MAX, //Eastern most col
            false => self.data[index + 1],
        };
        let south = match (index / self.width) + 1 < self.height {
            true => self.data[index + self.width],
            false => u8::MAX, //South row
        };
        let west = match index % self.width == 0 {
            true => u8::MAX, //Western most col
            false => self.data[index - 1],
        };

        let min = north.min(east.min(south.min(west)));
        let point = self.data[index];

        if point < min {
            point + 1
        } else {
            0
        }
    }
}