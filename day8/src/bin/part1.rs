use std::fs;

const FILE: &str = "prueba";

fn main() {
    let content = fs::read_to_string(FILE).unwrap();
    let input: Vec<&str> = content.lines().collect();
    let res = solve(input);
    println!("{}", res);
}

fn solve(arr: Vec<&str>) -> i32 {
    let instructions = *arr.first().unwrap();
    let mut n = arr.clone();
    n.remove(0);

    for i in n {
        println!("{}", i);
    }
    1
}
