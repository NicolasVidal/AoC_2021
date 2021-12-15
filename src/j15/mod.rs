use std::collections::HashMap;


#[derive(Clone)]
struct Node {
    total_cost: isize,
    row: usize,
    col: usize,
    closed: bool,
}

fn a_star(s: &str) -> usize {

    let mut grid = Vec::new();

    for line in s.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c as u8 - 48);
        }
        grid.push(row);
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let start_node = Node {
        total_cost: 0,
        row: 0,
        col: 0,
        closed: false
    };

    let mut known_nodes = HashMap::with_capacity(rows * cols);
    known_nodes.insert((start_node.row, start_node.col), start_node);

    loop {
        let min_node = known_nodes
            .iter_mut()
            .filter(|(_, n)| !n.closed)
            .min_by_key(|(_, n)|n.total_cost).unwrap().1;

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

fn update_neighbour(grid: &mut Vec<Vec<u8>>, known_nodes: &mut HashMap<(usize, usize), Node>, min_node: &Node, n_row: usize, n_col: usize) {
    let neighbour = known_nodes.entry((n_row, n_col))
        .or_insert(Node {
            total_cost: isize::MAX,
            row: n_row,
            col: n_col,
            closed: false
        });
    if neighbour.total_cost - grid[n_row][n_col] as isize > min_node.total_cost {
        neighbour.total_cost = min_node.total_cost + grid[n_row][n_col] as isize;
    }
}


pub fn _p1(s: &str) -> usize {
    a_star(s)
}

pub fn p1() -> usize {
    _p1(include_str!("j15.txt"))
}

pub fn _p2(s: &str) -> usize {
    a_star(s)
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
        assert_eq!(2188189693529, _p2(include_str!("j15_test.txt")));
        assert_eq!(2884513602164, _p2(include_str!("j15.txt")));
    }
}