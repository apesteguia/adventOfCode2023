use std::collections::HashMap;
use std::fs;

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let input: Vec<&str> = content.split('\n').collect();
    let mut count = 0;
    let mut aux = String::new();

    for i in input {
        aux.push(find_num(i.to_string()));
        aux.push(find_last_num(i.to_string()));
        //println!("{}", aux);
        count += aux.parse::<i32>().unwrap();
        aux = "".to_string();
    }

    println!("{}", count);
}

fn find_num(str: String) -> char {
    let n = convert_string(str);
    println!("{}", n);
    if n.is_empty() {
        return '0';
    }
    for i in n.chars() {
        if i.is_numeric() {
            return i;
        }
    }
    '0'
}

fn find_last_num(str: String) -> char {
    let n = convert_string(str);
    println!("{}", n);
    if n.is_empty() {
        return '0';
    }
    for i in n.chars().rev() {
        if i.is_numeric() {
            return i;
        }
    }
    '0'
}

fn convert_string(n: String) -> String {
    let nums: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut res = n.clone();
    let mut aux: Vec<usize> = Vec::new();

    for (i, &f) in nums.iter().enumerate() {
        if n.contains(f) {
            aux.push(i);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_string() {
        assert_eq!(convert_string("onetwothree".to_string()), "123".to_string());
    }
}
