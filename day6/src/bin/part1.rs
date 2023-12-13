use std::fs;

const FILE: &str = "input";

fn main() {
    let content = fs::read_to_string(FILE).unwrap();
    let input: Vec<&str> = content.lines().collect();
    let res = solve(input);
    println!("{}", res);
}

fn solve(arr: Vec<&str>) -> i32 {
    arr.first()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .zip(
            arr.last()
                .unwrap()
                .split(':')
                .last()
                .unwrap()
                .split_whitespace(),
        )
        .map(|(x, y)| (x, y))
        .map(|f| {
            let mut idx = 0;
            for i in 0..f.0.parse().unwrap() {
                if (f.0.parse::<i32>().unwrap() - i) * i > f.1.parse::<i32>().unwrap() {
                    idx += 1
                }
            }
            println!("{}", idx);
            idx
        })
        .product::<i32>()
}
