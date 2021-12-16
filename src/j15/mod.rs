use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::rc::Rc;

#[derive(Eq, PartialEq, Clone, Debug)]
struct AStarNode {
    cost: isize,
    total_cost: isize,
    row: usize,
    col: usize,
}

impl PartialOrd<Self> for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.total_cost.partial_cmp(&other.total_cost) {
            None => None,
            Some(o) => Some(match o {
                Ordering::Less => o,
                Ordering::Equal => {
                    let o = self.row.partial_cmp(&other.row).unwrap();
                    match o {
                        Ordering::Less => o,
                        Ordering::Equal => self.col.partial_cmp(&other.col).unwrap(),
                        Ordering::Greater => o
                    }
                }
                Ordering::Greater => o
            })
        }
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.total_cost.cmp(&other.total_cost) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                let o = self.row.cmp(&other.row);
                match o {
                    Ordering::Less => o,
                    Ordering::Equal => self.col.cmp(&other.col),
                    Ordering::Greater => o
                }
            }
            Ordering::Greater => Ordering::Greater
        }
    }
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

    let original_rows = grid.len();
    let original_cols = grid[0].len();

    let rows = if enlarge_grid { original_rows * 5 } else { original_rows };
    let cols = if enlarge_grid { original_cols * 5 } else { original_cols };

    let mut nodes_to_explore = BTreeSet::new();

    let mut nodes = Vec::with_capacity(rows);
    for row in 0..rows {
        let mut row_vec = Vec::with_capacity(cols);
        for col in 0..cols {
            let sub_grid_row = row / original_rows;
            let sub_grid_col = col / original_cols;
            let original_value = grid[row % original_rows][col % original_cols] as isize;

            let real_value = ((original_value - 1) + sub_grid_row as isize + sub_grid_col as isize) % 9 + 1;


            let node = Rc::new(AStarNode {
                cost: real_value,
                total_cost: if row == 0 && col == 0 { 0isize } else { isize::MAX },
                row,
                col,
            });
            nodes_to_explore.insert(node.clone());

            row_vec.push(node);
        }
        nodes.push(row_vec);
    }

    loop {
        let min_node = nodes_to_explore.iter().next().unwrap().clone();

        let min_node = nodes_to_explore.take(&min_node).unwrap();

        if min_node.col == cols - 1 && min_node.row == rows - 1 {
            return min_node.total_cost as usize;
        }

        if min_node.col > 0 {
            update_neighbour(&mut nodes_to_explore,
                             &mut nodes,
                             &min_node,
                             min_node.row,
                             min_node.col - 1);
        }
        if min_node.row > 0 {
            update_neighbour(
                &mut nodes_to_explore,
                &mut nodes,
                &min_node,
                min_node.row - 1,
                min_node.col);
        }
        if min_node.col < cols - 1 {
            update_neighbour(
                &mut nodes_to_explore,
                &mut nodes,
                &min_node,
                min_node.row,
                min_node.col + 1);
        }
        if min_node.row < rows - 1 {
            update_neighbour(
                &mut nodes_to_explore,
                &mut nodes,
                &min_node,
                min_node.row + 1,
                min_node.col);
        }
    }
}

fn update_neighbour(nodes_to_explore: &mut BTreeSet<Rc<AStarNode>>,
                    nodes: &mut Vec<Vec<Rc<AStarNode>>>,
                    min_node: &AStarNode,
                    n_row: usize, n_col: usize) {
    let neighbour = &nodes[n_row][n_col];

    if neighbour.total_cost - neighbour.cost > min_node.total_cost {
        nodes_to_explore.remove(neighbour);
        let updated_node = Rc::new(AStarNode {
            total_cost: min_node.total_cost + neighbour.cost,
            row: n_row,
            col: n_col,
            cost: neighbour.cost,
        });

        nodes_to_explore.insert(updated_node.clone());
        nodes[n_row][n_col] = updated_node;
    }

    // let real_rows = grid.len();
    // let real_cols = grid[0].len();
    //
    // let sub_grid_row = n_row / real_rows;
    // let sub_grid_col = n_col / real_cols;
    // let original_value = grid[n_row % real_rows][n_col % real_cols] as isize;
    //
    // let real_value = ((original_value - 1) + sub_grid_row as isize + sub_grid_col as isize) % 9 + 1;
    //
    // if neighbour.total_cost - real_value as isize > min_node.total_cost {
    //     neighbour.total_cost = min_node.total_cost + real_value as isize;
    // }
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