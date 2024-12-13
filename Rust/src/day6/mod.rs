use std::collections::HashSet;

pub fn solve1(input: &str) {
    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (rows, cols) = (matrix.len(), matrix[0].len());

    let mut guard_pos: (isize, isize) = get_guard_pos(&matrix, rows, cols);

    let mut guard_dir: (isize, isize) = (-1, 0);

    let mut out_of_bounds = false;

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert(guard_pos);

    while !out_of_bounds {
        matrix[guard_pos.0 as usize][guard_pos.1 as usize] = 'X';

        let next_step = (
            guard_pos.0 as isize + guard_dir.0,
            guard_pos.1 as isize + guard_dir.1,
        );
        let (r_n, c_n) = next_step;
        println!("Next Step {:?}", (next_step.0, next_step.1));

        if let Some(row) = matrix.get(r_n as usize) {
            if let Some(&ch) = row.get(c_n as usize) {
                if ch == '#' {
                    guard_dir = turn_right(guard_dir);
                }

                visited.insert(guard_pos);

                guard_pos = (
                    guard_pos.0 as isize + guard_dir.0,
                    guard_pos.1 as isize + guard_dir.1,
                );
            } else {
              visited.insert(guard_pos);
              out_of_bounds = true;
            }
        } else {
            visited.insert(guard_pos);
            out_of_bounds = true;
        }

        // for row in &matrix {
        //     println!("{:?}", row)
        // }
    }

    println!("{}", visited.len())
}

fn turn_right(direction: (isize, isize)) -> (isize, isize) {
    (direction.1, -direction.0)
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
  let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

  let (rows, cols) = (matrix.len(), matrix[0].len());

  let mut guard_pos: (isize, isize) = get_guard_pos(&matrix, rows, cols);

  let mut guard_dir: (isize, isize) = (-1, 0);

  let mut out_of_bounds = false;

  let mut visited: HashSet<(isize, isize)> = HashSet::new();
  visited.insert(guard_pos);

  while !out_of_bounds {

      let next_step = (
          guard_pos.0 as isize + guard_dir.0,
          guard_pos.1 as isize + guard_dir.1,
      );
      let (r_n, c_n) = next_step;
      println!("Next Step {:?}", (next_step.0, next_step.1));

      if let Some(row) = matrix.get(r_n as usize) {
          if let Some(&ch) = row.get(c_n as usize) {
              if ch == '#' {
                  guard_dir = turn_right(guard_dir);
                  matrix[guard_pos.0 as usize][guard_pos.1 as usize] = '+';

              }

              visited.insert(guard_pos);

              guard_pos = (
                  guard_pos.0 as isize + guard_dir.0,
                  guard_pos.1 as isize + guard_dir.1,
              );
          } else {
            visited.insert(guard_pos);
            out_of_bounds = true;
          }
      } else {
          visited.insert(guard_pos);
          out_of_bounds = true;
      }

    }
    
    for row in &matrix {
        println!("{:?}", row)
    }
  println!("{}", visited.len())
}

