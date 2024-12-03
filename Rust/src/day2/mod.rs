pub fn solve1(input: &str) {
  let mut sum = 0;

  for line in input.lines() {
      let int_levels: Vec<i32> = line
          .split_whitespace()
          .map(|l| l.parse::<i32>().unwrap_or_else(|_| panic!("Invalid integer: {}", l)))
          .collect();

      if is_safe(&int_levels) {
          sum += 1;
      }
  }

  println!("{}", sum);
}

pub fn solve2(input: &str) {
  let mut sum = 0;

  for line in input.lines() {
      let int_levels: Vec<i32> = line
          .split_whitespace()
          .map(|l| l.parse::<i32>().unwrap_or_else(|_| panic!("Invalid integer: {}", l)))
          .collect();

      if is_safe_2(&int_levels) {
          sum += 1;
      }
  }

  println!("{}", sum);
}

fn is_safe(levels: &[i32]) -> bool {
  let mut direction = 0; 
  for window in levels.windows(2) {
      let diff = window[1] - window[0];

      if diff.abs() > 3 {
          return false; 
      }

      if diff > 0 {
          if direction == -1 {
              return false;
          }
          direction = 1;
      } else if diff < 0 {
          if direction == 1 {
              return false;
          }
          direction = -1;
      } else {
          return false;
      }
  }
  true
}

fn is_safe_2(levels: &[i32]) -> bool {
  let mut direction = 0; 
  for (i, window) in levels.windows(2).enumerate() {
      let diff = window[1] - window[0];

      if diff.abs() > 3 {
          return false;
      }

      if diff == 0 {
          if !can_become_safe(i, &levels) {
              return false
          }
      } else if diff > 0 {
          if direction == -1 && !can_become_safe(i, &levels) {
              return false;
          }
          direction = 1;
      } else if diff < 0 {
          if direction == 1 && !can_become_safe(i, &levels) {
              return false;
          }
          direction = -1;
      }
  }
  true
}

fn can_become_safe(index: usize, levels: &[i32]) -> bool {
  if index == 0 || index >= levels.len() - 1 {
      return false;
  }

  let prev = levels[index - 1];
  let next = levels[index + 1];

  let diff = next - prev;

  if diff == 0 || diff.abs() > 3 {
      return false;
  }

  true
}