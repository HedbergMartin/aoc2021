use std::str::Lines;


fn main() {
    let mut lines = include_str!("input.txt").lines();

    let numbers: Vec<i32> = lines.next().unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect();

    let mut boards: Vec<BingoBoard> = Vec::new();
    while lines.next().is_some() {
        boards.push(BingoBoard::new(&mut lines));
    }

    for (i, n) in numbers.into_iter().enumerate() {
        for board in boards.iter_mut() {
            board.mark(n);
            
            if i >= 5 {
                if board.has_bingo() {
                    let score = board.calculate_score();
    
                    println!("Score of winning board: {}, last draw: {}, final score: {}", score, n, n * score);
                    return;
                }
            }
        }
    }
}

struct BingoBoard {
    board: Vec<(i32, bool)>,
}

impl BingoBoard {
    fn new(iter: &mut Lines<'_>) -> Self {
        let mut board: Vec<(i32, bool)> = Vec::new();

        for _ in 0..5 {
            let mut line: Vec<(i32, bool)> = iter.next().unwrap().split_whitespace().map(|x| (x.parse::<i32>().unwrap(), false)).collect();
            board.append(&mut line);
        }

        Self {board}
    }

    fn has_bingo(&self) -> bool {
        for x in 0..5 as usize {
            let mut failed = false;
            for y in 0..5 as usize {
                if !self.is_marked(x, y) {
                    failed = true;
                    break;
                }
            }

            if !failed {
                return true;
            }
        }

        
        for y in 0..5 as usize {
            let mut failed = false;
            for x in 0..5 as usize {
                if !self.is_marked(x, y) {
                    failed = true;
                    break;
                }
            }
            
            if !failed {
                return true;
            }
        }

        false
    }

    fn is_marked(&self, x: usize, y: usize) -> bool {
        self.board[x + (y * 5)].1
    }

    fn mark(&mut self, number: i32) {
        for e in &mut self.board {
            if e.0 == number {
                e.1 = true;
                return;
            }
        }
    }

    fn calculate_score(&self) -> i32 {
        let mut score = 0;
        for e in self.board.iter() {
            if e.1 == false {
                score += e.0;
            }
        }

        score
    }
}