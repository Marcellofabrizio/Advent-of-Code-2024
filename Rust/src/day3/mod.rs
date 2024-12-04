use regex::Regex;

pub fn solve1(input: &String) {
    let mut iter = input.split("").into_iter().peekable();
    let mut lexer: String = String::new();
    let mut sum_enabled = true;

    while let Some(mut token) = iter.next() {
        if token == "" {
            continue;
        };

        println!("Current: {}", token);

        match token {
            "d" => {
                lexer = String::new();
                lexer += token;
                token = iter.next().unwrap();
                if token == "o" {
                    lexer += token;
                    token = iter.next().unwrap();
                    if token == "(" {
                        lexer += token;
                        token = iter.next().unwrap();
                        if token == ")" {
                            lexer += token;
                        }
                    }
                }

                if token == "n" {
                    lexer += token;
                    token = iter.next().unwrap();
                    if token == "'" {
                        lexer += token;
                        token = iter.next().unwrap();
                        if token == "t" {
                            lexer += token;
                            token = iter.next().unwrap();
                            if token == "(" {
                                lexer += token;
                                token = iter.next().unwrap();
                                if token == ")" {
                                    lexer += token;
                                }
                            }
                        }
                    } else {
                        continue;
                    }
                }

                println!("lexer {}", lexer);

                match lexer.as_str() {
                    "don't()" => sum_enabled = false,
                    "do()" => sum_enabled = true,
                    _ => (),
                }
            }
            "m" => {
                if sum_enabled {
                    lexer = String::new();
                    lexer += token;

                    token = iter.next().unwrap();

                    if token == "u" {
                        token = iter.next().unwrap();
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn solve1_lexer(input: &String) {
  let mut iter = input.split("").into_iter().peekable();
  let mut lexer: String = String::new();
  let mut sum_enabled = true;

  while let Some(mut token) = iter.next() {
      if token == "" {
          continue;
      };

      println!("Current: {}", token);

      match token {
          "d" => {
              lexer = String::new();
              lexer += token;
              token = iter.next().unwrap();
              if token == "o" {
                  lexer += token;
                  token = iter.next().unwrap();
                  if token == "(" {
                      lexer += token;
                      token = iter.next().unwrap();
                      if token == ")" {
                          lexer += token;
                      }
                  }
              }

              if token == "n" {
                  lexer += token;
                  token = iter.next().unwrap();
                  if token == "'" {
                      lexer += token;
                      token = iter.next().unwrap();
                      if token == "t" {
                          lexer += token;
                          token = iter.next().unwrap();
                          if token == "(" {
                              lexer += token;
                              token = iter.next().unwrap();
                              if token == ")" {
                                  lexer += token;
                              }
                          }
                      }
                  } else {
                      continue;
                  }
              }

              println!("lexer {}", lexer);

              match lexer.as_str() {
                  "don't()" => sum_enabled = false,
                  "do()" => sum_enabled = true,
                  _ => (),
              }
          }
          "m" => {
              if sum_enabled {
                  lexer = String::new();
                  lexer += token;

                  token = iter.next().unwrap();

                  if token == "u" {
                      token = iter.next().unwrap();
                  }
              }
          }
          _ => {}
      }
  }
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
                    let op1 = cap
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse::<i32>()
                        .unwrap_or_else(|_| {
                            panic!(
                                "Invalid integer for value 1: {}",
                                cap.get(1).unwrap().as_str()
                            )
                        });
                    let op2 = cap
                        .get(2)
                        .unwrap()
                        .as_str()
                        .parse::<i32>()
                        .unwrap_or_else(|_| {
                            panic!(
                                "Invalid integer for value 2: {}",
                                cap.get(2).unwrap().as_str()
                            )
                        });
                    sum += op1 * op2;
                }
                _ => {}
            }
        }
    }

    println!("{}", sum);
}
