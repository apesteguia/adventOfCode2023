use std::fs;

const FILE: &str = "prueba";

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let input: Vec<&str> = contents.lines().collect();
    let res = parse_input(input);
    println!("{}", res);
}

fn parse_input(arr: Vec<&str>) -> i32 {
    arr.iter()
        .map(|&line| {
            let numbers: Vec<&str> = line.trim().split(':').last().unwrap().split('|').collect();
            winning(numbers)
        })
        .sum()
}

fn winning(arr: Vec<&str>) -> i32 {
    let a: Vec<&str> = arr.last().unwrap().split_whitespace().collect();

    let mut i = 0;
    arr.first().unwrap().split_whitespace().for_each(|f| {
        if a.contains(&f) {
            if i == 0 {
                println!("Empiezo {}", f);
                i += 1;
            } else {
                i *= 2;
                println!("{} count: {}", f, i);
            }
        }
    });

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winning() {
        assert_eq!(
            winning(Vec::from([
                "41 48 83 86 17",
                "83 86 6 31 17 9 48 53 17 9 48 53 17 9 48 53 17 9 48 53"
            ])),
            8
        )
    }
}
