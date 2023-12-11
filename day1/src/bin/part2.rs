use std::collections::BTreeMap;
use std::fs;

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let input: Vec<&str> = content.split('\n').collect();
    let mut count = 0;
    let mut aux = String::new();

    for i in input {
        aux = convert_string(i.to_string());
        count += aux.parse::<i32>().unwrap();
        println!("{}, {}", i, aux);
        aux = "".to_string();
    }

    println!("{}", count);
}

fn find_num(str: &str) -> char {
    if str.is_empty() {
        return '0';
    }
    for _i in str.chars() {
        for i in str.chars() {
            if i.is_numeric() {
                return i;
            }
        }
    }
    '0'
}

fn convert_string(n: String) -> String {
    let mut numbers_map: BTreeMap<char, &str> = BTreeMap::new();
    numbers_map.insert('1', "one");
    numbers_map.insert('2', "two");
    numbers_map.insert('3', "three");
    numbers_map.insert('4', "four");
    numbers_map.insert('5', "five");
    numbers_map.insert('6', "six");
    numbers_map.insert('7', "seven");
    numbers_map.insert('8', "eight");
    numbers_map.insert('9', "nine");

    let mut res: Vec<char> = Vec::new();
    let mut idx: Vec<usize> = Vec::new();
    let mut returned: Vec<char> = Vec::new();

    for (&i, &v) in &numbers_map {
        if let Some(_idx) = n.find(v) {
            res.push(i);
            idx.push(_idx);
        }
    }

    if res.is_empty() {
        res.push(find_num(&n.clone()));
        res.push(find_num(&n.clone().chars().rev().collect::<String>()));
        return res.iter().collect();
    }

    let sorted_indices: Vec<usize> = (0..idx.len()).collect();
    let mut zipped: Vec<_> = sorted_indices.into_iter().zip(idx.iter()).collect();
    zipped.sort_by_key(|&(_, idx)| idx);

    let sorted_idx: Vec<usize> = zipped.iter().map(|&(i, _)| i).collect();
    let sorted_res: Vec<char> = sorted_idx.iter().map(|&i| res[i]).collect();

    let min_idx = *idx.iter().min().unwrap();
    let max_idx = *idx.iter().max().unwrap();

    for (i, f) in n.chars().enumerate() {
        if f.is_numeric() {
            if i < min_idx {
                returned.push(f);
            } else {
                returned.push(*sorted_res.first().unwrap());
            }
            break;
        }
    }

    for (i, f) in n.chars().rev().enumerate() {
        if f.is_numeric() {
            if i >= min_idx {
                returned.push(f);
            } else {
                returned.push(*sorted_res.last().unwrap());
            }
            break;
        }
    }

    if returned.is_empty() {
        returned.push(*sorted_res.first().expect("FALLO"));
        returned.push(*sorted_res.last().expect("FALLO"));
    }

    returned.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_string() {
        assert_eq!(
            convert_string("ppjvndvknbtpfsncplmhhrlh5".to_string()),
            "55".to_string()
        );
    }
}
