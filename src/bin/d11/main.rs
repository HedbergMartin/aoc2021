
fn main() {
	let lines = include_str!("input.txt").lines();

	let mut grid1 = Grid::new();
	let mut grid2 = Grid::new();

	for l in lines {
        grid1.expand(l);
        grid2.expand(l);
    }

	let mut flashes = 0;
	for _ in 0..100 {
		flashes += grid1.step();
	}
	
	println!("Total flashes {}", flashes);

	let squids = grid2.data.len();
	let mut step = 0;
	loop {
		let flash = grid2.step();
		step += 1;
		if squids == flash {
			break;
		}
	}

	println!("First step when all flash is {}", step);
}

struct Grid {
	data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
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

	fn step(&mut self) -> usize {
		let mut maxed: Vec<usize> = Vec::new();
		for squid in self.data.iter_mut().enumerate() {
			*squid.1 += 1;
			if *squid.1 > 9 {
				maxed.push(squid.0);
			}
		}

		let mut flashes = 0;
		for ud in maxed.into_iter() {
			flashes += self.flash((ud % self.width) as isize, (ud / self.width) as isize);
		}

		flashes
	}

	fn flash(&mut self, x: isize, y: isize) -> usize {
		if let Some(index) = self.as_index(x, y) {
			if self.data[index] > 0 {
				self.data[index] += 1;
			}
			if self.data[index as usize] > 9 {
				self.data[index as usize] = 0;

				let mut flashes = 1;
				for y_offset in -1..=1 {
					for x_offset in -1..=1 {
						flashes += self.flash(x + x_offset, y + y_offset);
					}
				}

				return flashes;
			}
		}

		0
	}

	fn as_index(&self, x: isize, y: isize) -> Option<usize> {
		if x >= 0 && x < self.width as isize {
			if y >= 0 && y < self.height as isize {
				return Some(x as usize + y as usize * self.width);
			}
		}

		None
	}

	// fn flash(&mut self, index: isize) -> usize {
	// 	if index >= 0 && index < self.data.len() as isize { aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
	// 		if self.data[index as usize] > 0 {
	// 			self.data[index as usize] += 1;
	// 		}
	// 		if self.data[index as usize] > 9 {
	// 			self.data[index as usize] = 0;

	// 			let mut flashes = 1;
	// 			for y in -1..=1 {
	// 				for x in -1..=1 {
	// 					flashes += self.flash(index + x + (self.width as isize * y))
	// 				}
	// 			}

	// 			return flashes;
	// 		}
	// 	}

	// 	0
	// }

	#[allow(dead_code)]
    fn print_grid(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.data[x + (self.width * y)]);
            }
            println!("");
        }
    }
}