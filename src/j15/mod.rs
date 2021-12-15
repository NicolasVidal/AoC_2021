use std::collections::HashMap;


#[derive(Clone)]
struct Node {
    total_cost: isize,
    row: usize,
    col: usize,
    closed: bool,
}

fn a_star(s: &str, enlarge_grid: bool) -> usize {
    let mut grid = Vec::new();

    for line in s.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c as u8 - 48);
        }
        grid.push(row);
    }

    let rows = if enlarge_grid { grid.len() * 5 } else { grid.len() };
    let cols = if enlarge_grid { grid[0].len() * 5 } else { grid[0].len() };

    let start_node = Node {
        total_cost: 0,
        row: 0,
        col: 0,
        closed: false,
    };

    let mut known_nodes = HashMap::with_capacity(rows * cols);
    known_nodes.insert((start_node.row, start_node.col), start_node);

    loop {
        let min_node = known_nodes
            .iter_mut()
            .filter(|(_, n)| !n.closed)
            .min_by_key(|(_, n)| n.total_cost
                // heuristic seems to harm performances
                // + (n.row as isize - rows as isize + 1).abs()
                // + (n.col as isize - cols as isize + 1).abs()
            ).unwrap().1;

        min_node.closed = true;

        let min_node = min_node.clone();

        if min_node.col == cols - 1 && min_node.row == rows - 1 {
            return min_node.total_cost as usize;
        }

        if min_node.col > 0 {
            update_neighbour(&mut grid, &mut known_nodes, &min_node,
                             min_node.row,
                             min_node.col - 1);
        }
        if min_node.row > 0 {
            update_neighbour(&mut grid, &mut known_nodes, &min_node,
                             min_node.row - 1,
                             min_node.col);
        }
        if min_node.col < cols - 1 {
            update_neighbour(&mut grid, &mut known_nodes, &min_node,
                             min_node.row,
                             min_node.col + 1);
        }
        if min_node.row < rows - 1 {
            update_neighbour(&mut grid, &mut known_nodes, &min_node,
                             min_node.row + 1,
                             min_node.col);
        }
    }
}

fn update_neighbour(grid: &mut Vec<Vec<u8>>, known_nodes: &mut HashMap<(usize, usize), Node>, min_node: &Node,
                    n_row: usize, n_col: usize) {
    let neighbour = known_nodes.entry((n_row, n_col))
        .or_insert(Node {
            total_cost: isize::MAX,
            row: n_row,
            col: n_col,
            closed: false,
        });
    let real_rows = grid.len();
    let real_cols = grid[0].len();

    let sub_grid_row = n_row / real_rows;
    let sub_grid_col = n_col / real_cols;
    let original_value = grid[n_row % real_rows][n_col % real_cols] as isize;

    let real_value = ((original_value - 1) + sub_grid_row as isize + sub_grid_col as isize) % 9 + 1;

    if neighbour.total_cost - real_value as isize > min_node.total_cost {
        neighbour.total_cost = min_node.total_cost + real_value as isize;
    }
}


pub fn _p1(s: &str) -> usize {
    a_star(s, false)
}

pub fn p1() -> usize {
    _p1(include_str!("j15.txt"))
}

pub fn _p2(s: &str) -> usize {
    a_star(s, true)
}

pub fn p2() -> usize {
    _p2(include_str!("j15.txt"))
}

#[cfg(test)]
mod j15_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(40, _p1(include_str!("j15_test.txt")));
        assert_eq!(673, _p1(include_str!("j15.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(315, _p2(include_str!("j15_test.txt")));
        assert_eq!(2893, _p2(include_str!("j15.txt")));
    }
}