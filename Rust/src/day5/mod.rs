use std::collections::HashMap;

pub fn solve1(input: &String) {
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let rules: Vec<&str> = split_input[0].lines().collect();
    let updates: Vec<Vec<u32>> = split_input[1]
        .lines()
        .map(|s| {
            s.split(',')
                .map(|num| {
                    num.parse::<u32>()
                        .unwrap_or_else(|_| panic!("Invalid integer: {}", num))
                })
                .collect()
        })
        .collect();

    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();

    for rule in &rules {
        let rule_pages: Vec<&str> = rule.split('|').collect();
        let first_page = rule_pages[0]
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("Invalid integer"));

        let second_page = rule_pages[1]
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("Invalid integer"));

        rules_map
            .entry(first_page)
            .or_insert_with(Vec::new)
            .push(second_page);
    }

    println!("{:?}", rules_map);

    let mut valid_updates: Vec<Vec<u32>> = Vec::new();

    for update in &updates {
        let update_size = update.len();
        let mut valid = true;
        for (i, page) in update.iter().enumerate() {
            for prev in 0..i {
                if rules_map.contains_key(&update[prev]) && rules_map[&update[prev]].contains(page)
                {
                    continue;
                } else {
                    valid = false;
                }
            }

            for next in i + 1..update_size {
                if next < update_size {
                    if rules_map.contains_key(page) && rules_map[&page].contains(&update[next]) {
                        continue;
                    } else {
                        valid = false;
                        break;
                    }
                }
            }
        }

        // println!(
        //     "Update {:?} is {}",
        //     update,
        //     if valid { "valid" } else { "not valid" }
        // );

        if valid {
            valid_updates.push((&update).to_vec());
        }
    }

    let sum: u32 = valid_updates.iter().map(|u| u[u.len() / 2]).sum();
    println!("{}", sum)
}

pub fn solve2(input: &String) {}
