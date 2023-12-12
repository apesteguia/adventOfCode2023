use std::fs;

const FILE: &str = "input";

fn main() {
    let content = fs::read_to_string(FILE).unwrap();
    let input: Vec<&str> = content.split('\n').collect();
    let res = parse_input(input);
    println!("{}", res);
}

fn parse_input(arr: Vec<&str>) -> i32 {
    arr.iter()
        .map(|&f| fewest_nums(f.split(':').last().unwrap()))
        .sum()
}

fn fewest_nums(n: &str) -> i32 {
    let mut green: Vec<i32> = Vec::new();
    let mut blue: Vec<i32> = Vec::new();
    let mut red: Vec<i32> = Vec::new();

    n.split(';').for_each(|f| {
        f.split(',').for_each(|e| {
            let x: Vec<&str> = e.split_whitespace().collect();
            if let Some(color) = x.last() {
                match *color {
                    "blue" => blue.push(x.first().unwrap().parse().unwrap()),
                    "red" => red.push(x.first().unwrap().parse().unwrap()),
                    "green" => green.push(x.first().unwrap().parse().unwrap()),
                    _ => println!("error"),
                }
            }
        });
    });
    let green_max = green.iter().max_by_key(|&x| x).copied().unwrap_or(0);
    let blue_max = blue.iter().max_by_key(|&x| x).copied().unwrap_or(0);
    let red_max = red.iter().max_by_key(|&x| x).copied().unwrap_or(0);

    green_max * blue_max * red_max
}
