
#[derive(Clone, Debug)]
enum Cell {
    Empty,
    East,
    South,
}


pub fn _p1(s: &'static str) -> usize {
    let mut grid = Vec::with_capacity(140);

    for line in s.lines() {
        let mut vec = Vec::with_capacity(140);
        for c in line.chars() {
            vec.push(match c {
                '>' => Cell::East,
                'v' => Cell::South,
                '.' => Cell::Empty,
                _ => panic!(),
            });
        }
        grid.push(vec)
    }
    let mut grid_copy = grid.clone();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut step = 0;
    loop {
        let mut changed = false;

        // for row in 0..rows {
        //     for col in 0..cols {
        //         print!("{}", match grid[row][col] {
        //             Cell::Empty => '.',
        //             Cell::East => '>',
        //             Cell::South => 'v',
        //         })
        //     }
        //     println!()
        // }
        // println!();

        // Move East
        for row in 0..rows {
            let mut col = 0;
            loop {
                if col >= cols {
                    break;
                }

                grid_copy[row][col] = grid[row][col].clone();
                match grid[row][col] {
                    Cell::East => {
                        let next_col = (col + 1) % cols;
                        match grid[row][next_col] {
                            Cell::Empty => {
                                changed = true;
                                grid_copy[row][col] = Cell::Empty;
                                grid_copy[row][next_col] = Cell::East;
                                col += 1;
                            }
                            Cell::East => {}
                            Cell::South => {}
                        }
                    }
                    _ => {}
                }

                col += 1;
            }
        }

        std::mem::swap(&mut grid, &mut grid_copy);

        // Move South
        for col in 0..cols {
            let mut row = 0;

            loop {
                if row >= rows {
                    break;
                }
                grid_copy[row][col] = grid[row][col].clone();
                match grid[row][col] {
                    Cell::South => {
                        let next_row = (row + 1) % rows;
                        match grid[next_row][col] {
                            Cell::Empty => {
                                changed = true;
                                grid_copy[row][col] = Cell::Empty;
                                grid_copy[next_row][col] = Cell::South;
                                row += 1;
                            }
                            Cell::East => {}
                            Cell::South => {}
                        }
                    }
                    _ => {}
                }
                row += 1;
            }
        }

        std::mem::swap(&mut grid, &mut grid_copy);

        step += 1;

        if !changed {
            break;
        }
    }

    step
}

pub fn p1() -> usize {
    _p1(include_str!("j25.txt"))
}

pub fn _p2(s: &'static str) -> usize {
    _p1(s)
}

pub fn p2() -> usize {
    _p2(include_str!("j25.txt"))
}

#[cfg(test)]
mod j25_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(58, _p1(include_str!("j25_test.txt")));
        assert_eq!(321, _p1(include_str!("j25.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(58, _p2(include_str!("j25_test.txt")));
        assert_eq!(321, _p2(include_str!("j25.txt")));
    }
}