use std::fs;

const FILE: &str = "input";
const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

fn main() {
    let content = fs::read_to_string(FILE).unwrap();
    let input: Vec<&str> = content.split('\n').collect();
    let res = parse_input(input);
    println!("{}", res);
}

fn parse_input(arr: Vec<&str>) -> i32 {
    arr.iter()
        .enumerate()
        .map(|(i, &f)| {
            let game = f.split(':').last().unwrap();
            if game.split(';').all(is_possible) {
                println!("Game: {} {}", i + 1, game);
                (i + 1) as i32
            } else {
                0
            }
        })
        .sum()
}

fn is_possible(n: &str) -> bool {
    n.split(',').all(|f| colors(f.split_whitespace().collect()))
}

fn colors(n: Vec<&str>) -> bool {
    match *n.last().unwrap() {
        "blue" => n.first().unwrap().parse::<i32>().unwrap() <= BLUE,
        "red" => n.first().unwrap().parse::<i32>().unwrap() <= RED,
        "green" => n.first().unwrap().parse::<i32>().unwrap() <= GREEN,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colors() {
        let a: Vec<&str> = Vec::from(["3", "green"]);
        assert!(colors(a));
    }

    #[test]
    fn test_is_possible() {
        assert!(is_possible("3 blue, 14 green, 4 blue"));
    }
}
