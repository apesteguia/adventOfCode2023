use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = content.split('\n').collect();
    let mut aux = String::new();
    let mut cont = 0;

    for i in input {
        aux.push(find_first_num(i.to_string()));
        aux.push(find_first_num(i.chars().rev().collect()));
        cont += aux.parse::<i32>().ok().unwrap();
        aux = "".to_string();
    }

    println!("{}", cont);
}

fn find_first_num(n: String) -> char {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_num() {
        assert_eq!(find_first_num("pqr3stu8dfa".to_string()), '3');
    }
}
