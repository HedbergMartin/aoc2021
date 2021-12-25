
type Coord = usize;
type Point = (Coord, Coord);
type Line = (Point, Point);

fn main() {
    let input_lines = include_str!("input.txt").lines();

    let lines: Vec<Line> = input_lines.map(|s| {
        let points = s.split_once(" -> ").unwrap();
        (parse_point(points.0), parse_point(points.1))
    }).collect();

    let mut grid = Grid::new(1000, 1000); // could have read from the file

    for l in lines.iter() {
        grid.add_line(l);
    }

    println!("Overlaps over 2: {}", grid.overlaps_in_grid(2));
}

fn parse_point(s: &str) -> Point {
    let point1str = s.split_once(',').unwrap();
    (point1str.0.parse::<Coord>().unwrap(), point1str.1.parse::<Coord>().unwrap())
}

struct Grid {
    grid: Vec<i32>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {grid: vec![0; width * height], width, height}
    }

    fn add_line(&mut self, line: &Line) {
        let mut dx = line.1.0 as i32 - line.0.0 as i32;
        let mut dy = line.1.1 as i32 - line.0.1 as i32;

        let steps = dx.abs().max(dy.abs());

        dx = dx.signum();
        dy = dy.signum();

        let mut x = line.0.0 as i32;
        let mut y = line.0.1 as i32;

        for _ in 0..=steps {
            self.grid[x as usize + (self.width * y as usize)] += 1;
            x += dx;
            y += dy;
        }
    }

    fn overlaps_in_grid(&self, overlaps: i32) -> i32 {
        let mut tot_overlaps = 0;
        for i in self.grid.iter() {
            if overlaps <= *i {
                tot_overlaps += 1;
            }
        }

        tot_overlaps
    }

    #[allow(dead_code)]
    fn print_grid(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.grid[x + (self.width * y)];
                if val > 0 {
                    print!("{}", val);
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}