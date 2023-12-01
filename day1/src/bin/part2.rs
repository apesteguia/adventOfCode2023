use std::collections::HashMap;
use std::fs;

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let input: Vec<&str> = content.split('\n').collect();
    let mut count = 0;
    let mut aux = String::new();

    for i in input {
        aux.push(find_first_num(i.to_string()));
        aux.push(find_last_num(i.to_string()));
        println!("{}", aux);
        count += aux.parse::<i32>().unwrap();
        aux = "".to_string();
    }

    println!("{}", count);
}

fn find_first_num(n: String) -> char {
    let nums: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let nums_map: HashMap<&str, char> = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]
    .into_iter()
    .collect();

    let mut nums_pos = n.len();
    let mut returned_char = '0';

    for (i, f) in n.chars().enumerate() {
        if f.is_numeric() {
            return f;
        }

        if i >= nums_pos {
            break;
        }

        if let Some((num, idx)) = nums
            .iter()
            .filter_map(|&word| n.find(word).map(|idx| (word, idx)))
            .min_by_key(|&(_, idx)| idx)
        {
            nums_pos = idx;
            returned_char = *nums_map.get(num).unwrap_or(&'0');
        }
    }

    returned_char
}
fn find_last_num(n: String) -> char {
    let nums: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let nums_map: HashMap<&str, char> = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]
    .into_iter()
    .collect();

    let mut nums_pos = n.len();
    let mut returned_char = '0';

    for (i, f) in n.chars().rev().enumerate() {
        if i >= nums_pos {
            break;
        }

        if let Some((num, idx)) = nums
            .iter()
            .rev()
            .filter_map(|&word| n.rfind(word).map(|idx| (word, idx)))
            .max_by_key(|&(_, idx)| idx)
        {
            nums_pos = idx;
            returned_char = *nums_map.get(num).unwrap_or(&'0');
        }

        if f.is_numeric() {
            returned_char = f;
        }
    }

    returned_char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_num() {
        assert_eq!(find_first_num("asdfasdfone3stu8dfa".to_string()), '1');
    }
}
