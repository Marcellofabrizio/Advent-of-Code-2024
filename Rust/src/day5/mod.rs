use std::collections::HashMap;

pub fn solve1(input: &String) {
    let (rules_map, updates) = parse_input(input);

    let valid_updates: Vec<Vec<u32>> = updates
        .iter()
        .filter(|update| is_valid(update, &rules_map))
        .cloned()
        .collect();

    let sum: u32 = valid_updates.iter().map(|u| u[u.len() / 2]).sum();
    println!("{}", sum);
}

pub fn solve2(input: &String) {
    let (rules_map, updates) = parse_input(input);

    let invalid_updates: Vec<Vec<u32>> = updates
        .iter()
        .filter(|update| !is_valid(update, &rules_map))
        .cloned()
        .collect();

    let fixed_updates: Vec<Vec<u32>> = invalid_updates
        .iter()
        .map(|update| fix_invalid_update(update, &rules_map))
        .collect();

    let sum: u32 = fixed_updates.iter().map(|u| u[u.len() / 2]).sum();
    println!("{}", sum);
}

fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let rules = split_input[0].lines();
    let updates: Vec<Vec<u32>> = split_input[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| {
                    num.parse::<u32>()
                        .unwrap_or_else(|_| panic!("Invalid integer"))
                })
                .collect()
        })
        .collect();

    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();

    for rule in rules {
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

    (rules_map, updates)
}

fn is_valid(update: &[u32], rules_map: &HashMap<u32, Vec<u32>>) -> bool {
    for (i, &page) in update.iter().enumerate() {
        for prev in 0..i {
            if let Some(allowed_pages) = rules_map.get(&update[prev]) {
                if !allowed_pages.contains(&page) {
                    return false;
                }
            }
        }

        for next in i + 1..update.len() {
            if let Some(allowed_pages) = rules_map.get(&page) {
                if !allowed_pages.contains(&update[next]) {
                    return false;
                }
            }
        }
    }
    true
}

fn fix_invalid_update(update: &[u32], rules_map: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut new_array = update.to_vec();

    for i in 0..new_array.len() {
        let page = new_array[i];

        for prev in 0..i {
            if let Some(allowed_pages) = rules_map.get(&new_array[prev]) {
                if !allowed_pages.contains(&page) {
                    new_array.swap(prev, i);
                }
            }
        }
    }

    new_array
}
