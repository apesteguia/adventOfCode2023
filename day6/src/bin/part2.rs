use std::fs;

const FILE: &str = "input";

fn main() {
    let content = fs::read_to_string(FILE).unwrap();
    let input: Vec<&str> = content.lines().collect();
    let res = solve(input);
    println!("{}", res);
}

fn solve(arr: Vec<&str>) -> i32 {
    let a = parse(arr.first().unwrap());
    let b = parse(arr.last().unwrap());
    let mut idx = 0;
    for i in 0..a {
        if (a - i) * i > b {
            idx += 1;
        }
    }
    idx
}

fn parse(n: &str) -> u64 {
    let a: String = n
        .split(':')
        .last()
        .unwrap_or("") // Si el split no produce ningún resultado, usar una cadena vacía
        .split_whitespace()
        .map(|f| f.to_string())
        .collect();
    println!("{}", a);
    a.parse().unwrap()
}
