use std::collections::HashSet;

pub fn solve1(input: &str) {
    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (rows, cols) = (matrix.len(), matrix[0].len());

    let mut guard_pos: (isize, isize) = get_guard_pos(&matrix, rows, cols);
    let mut guard_dir = 0;

    let dirs: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert(guard_pos);

    loop {
        matrix[guard_pos.0 as usize][guard_pos.1 as usize] = 'X';

        let next_step = (
            guard_pos.0 + dirs[guard_dir].0,
            guard_pos.1 + dirs[guard_dir].1,
        );

        let (r_n, c_n) = next_step;

        if (0 <= r_n && r_n < rows as isize) && (0 <= c_n && c_n < cols as isize) {
            if matrix[r_n as usize][c_n as usize] == '#' {
                guard_dir = (guard_dir + 1) % 4;
            } else {
                guard_pos = (r_n, c_n);
                visited.insert((guard_pos.0, guard_pos.1));
            }
        } else {
            break;
        }
    }

    println!("{}", visited.len())
}

fn get_guard_pos(matrix: &Vec<Vec<char>>, rows: usize, cols: usize) -> (isize, isize) {
    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col] == '^' {
                return (row as isize, col as isize);
            }
        }
    }

    return (0, 0);
}

pub fn solve2(input: &str) {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (rows, cols) = (matrix.len(), matrix[0].len());

    let mut guard_pos: (isize, isize) = get_guard_pos(&matrix, rows, cols);
    let start_pos = guard_pos;
    let mut guard_dir: usize = 0;

    let mut sum = 0;

    let dirs: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    loop {
        let next_step = (
            guard_pos.0 as isize + dirs[guard_dir].0,
            guard_pos.1 as isize + dirs[guard_dir].1,
        );

        let (r_n, c_n) = next_step;

        if (0 <= r_n && r_n < rows as isize) && (0 <= c_n && c_n < cols as isize) {
            if matrix[r_n as usize][c_n as usize] == '#' {
                guard_dir = (guard_dir + 1) % 4;
            }

            guard_pos = (
                guard_pos.0 + dirs[guard_dir].0,
                guard_pos.1 + dirs[guard_dir].1,
            );

            visited.insert((guard_pos.0 as usize, guard_pos.1 as usize));

            continue;
        } else {
            break;
        }
    }

    for (o_r, o_c) in visited {
        let (mut r, mut c) = start_pos;
        let mut dir: usize = 0;
        let mut visited_loop: HashSet<(isize, isize, usize)> = HashSet::new();

        loop {
            let dirs: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

            if visited_loop.contains(&(r, c, dir)) {
                sum += 1;
                break;
            }

            visited_loop.insert((r, c, dir));

            let r_n = r + dirs[dir].0;
            let c_n = c + dirs[dir].1;

            if (0 <= r_n && r_n < rows as isize) && (0 <= c_n && c_n < cols as isize) {
                if matrix[r_n as usize][c_n as usize] == '#'
                    || r_n == o_r as isize && c_n == o_c as isize
                {
                    dir = (dir + 1) % 4;
                } else {
                    r = r_n;
                    c = c_n;
                }
            } else {
                break;
            }
        }
    }

    println!("{}", sum)
}
