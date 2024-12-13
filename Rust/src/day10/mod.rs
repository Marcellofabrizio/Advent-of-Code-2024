use std::collections::HashSet;

pub fn solve1(input: &str) {
    let matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|s| s.to_digit(10)).collect())
        .collect();

    let (rows, cols) = (matrix.len(), matrix[0].len());

    let mut sum = 0;

    for row in 0..rows {
        for col in 0..cols {
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            if matrix[row][col] == 0 {
                sum += dfs(1, &matrix, (row, col), &mut visited);
            }
        }
    }

    println!("{}", sum);
}

pub fn dfs(
    part: u8,
    matrix: &Vec<Vec<u32>>,
    node: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> u32 {
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    if part == 1 {
        if visited.contains(&node) {
            return 0;
        }

        visited.insert(node);
    }

    if matrix[node.0][node.1] == 9 {
        return 1;
    }

    let mut paths = 0;

    for (d_r, d_c) in dirs {
        let comp_row = node.0 as isize + d_r;
        let comp_col = node.1 as isize + d_c;

        if let Some(r_i) = matrix.get(comp_row as usize) {
            if let Some(&c_i) = r_i.get(comp_col as usize) {
                if c_i as i32 - matrix[node.0][node.1] as i32 == 1 {
                    paths += dfs(
                        part,
                        matrix,
                        (comp_row as usize, comp_col as usize),
                        visited,
                    );
                }
            } else {
                continue;
            }
        }
    }

    return paths;
}

pub fn solve2(input: &str) {
    let matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|s| s.to_digit(10)).collect())
        .collect();

    let (rows, cols) = (matrix.len(), matrix[0].len());

    let mut sum = 0;

    for row in 0..rows {
        for col in 0..cols {
            let mut visited: HashSet<(usize, usize)> = HashSet::new();

            if matrix[row][col] == 0 {
                sum += dfs(2, &matrix, (row, col), &mut visited);
            }
        }
    }

    println!("{}", sum);
}
