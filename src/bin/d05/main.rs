use std::ops::RangeInclusive;

type Coord = usize;
type Point = (Coord, Coord);
type Line = (Point, Point);

fn main() {
    let input_lines = include_str!("input.txt").lines();

    let lines: Vec<Line> = input_lines.map(|s| {
        let points = s.split_once(" -> ").unwrap();
        (parse_point(points.0), parse_point(points.1))
    }).collect();

    let straight_lines: Vec<Line> = lines.into_iter().filter(|&(l1, l2)| {
        l1.0 == l2.0 || l1.1 == l2.1
    }).collect();

    let mut grid = Grid::new(1000, 1000);

    for line in straight_lines.iter() {
        grid.add_line(line);
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
        for x in Grid::abs_range(line.0.0, line.1.0) {
            for y in Grid::abs_range(line.0.1, line.1.1) {
                self.grid[x + (self.width * y)] += 1;
            }
        }
    }

    fn abs_range(a: usize, b: usize) -> RangeInclusive<usize> {
        if a < b {
            a..=b
        } else {
            b..=a
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
                print!("{}", self.grid[x + (self.width * y)]);
            }
            println!("");
        }
    }
}