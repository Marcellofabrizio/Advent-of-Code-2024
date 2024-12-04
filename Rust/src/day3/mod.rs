use regex::Regex;

pub fn solve1(input: &String) {
    let re = Regex::new(r"mul\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();
    let sum: i32 = re.captures_iter(input)
    .map(|caps| {
        let op1 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap_or_else(|_| {
            panic!("Invalid integer for value 1: {}", caps.get(1).unwrap().as_str())
        });
        let op2 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap_or_else(|_| {
            panic!("Invalid integer for value 2: {}", caps.get(2).unwrap().as_str())
        });
        op1 * op2
    })
    .sum();

println!("{}", sum);

}

pub fn solve2(input: &String) {

  let re = Regex::new(r"(mul\(\s*(\d+)\s*,\s*(\d+)\s*\)|don't()|do())").unwrap();
  let mut sum_enabled = true;
  let mut sum = 0;
  for cap in re.captures_iter(input) {
    if let Some(op) = cap.get(0) {
        match op.as_str() {
            "do" => sum_enabled = true,
            "don't" => sum_enabled = false,
            _ if sum_enabled => {
                let op1 = cap.get(1).unwrap().as_str().parse::<i32>().unwrap_or_else(|_| {
                    panic!("Invalid integer for value 1: {}", cap.get(1).unwrap().as_str())
                });
                let op2 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap_or_else(|_| {
                    panic!("Invalid integer for value 2: {}", cap.get(2).unwrap().as_str())
                });
                sum += op1 * op2;
            }
            _ => {}
        }
    }
}

  println!("{}", sum);
}
