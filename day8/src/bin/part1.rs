use std::fs;

const FILE: &str = "input";
//19783

fn main() {
    let content = fs::read_to_string(FILE).unwrap();
    let input: Vec<&str> = content.lines().collect();
    let res = solve(input);
    println!("{}", res);
}

fn solve(arr: Vec<&str>) -> u32 {
    let instructions: Vec<char> = arr.first().unwrap().chars().collect();
    let mut n = arr.clone();
    n.remove(0);
    n.remove(0);

    let indices: Vec<&str> = n
        .iter()
        .map(|&f| f.split('=').next().unwrap().trim())
        .collect();

    let left_right: Vec<(&str, &str)> = n
        .iter()
        .map(|&f| {
            let parts: Vec<&str> = f.split('=').last().unwrap().trim().split(',').collect();
            match parts.as_slice() {
                [left, right] => (
                    left.split('(').last().unwrap(),
                    right.split(')').next().unwrap().trim(),
                ),
                _ => panic!("Invalid input format"),
            }
        })
        .collect();

    let mut looked_element = "AAA";
    let mut idx;
    let mut ins_idx = 0;
    let mut count: u32 = 0;
    let mut founded = "";

    idx = buscar_elemento(&indices, looked_element).unwrap();
    while founded != "ZZZ" {
        if instructions[ins_idx] == 'L' {
            looked_element = left_right[idx].0;
            print!("L ");
        } else {
            looked_element = left_right[idx].1;
            print!("R ");
        }
        idx = buscar_elemento(&indices, looked_element).unwrap();

        founded = indices[idx];
        println!(" {} {}", founded, ins_idx);
        ins_idx += 1;
        if ins_idx >= instructions.len() {
            ins_idx = 0;
        }
        count += 1;
    }

    count
}

fn buscar_elemento(arr: &[&str], elemento: &str) -> Option<usize> {
    arr.iter().position(|&x| x == elemento)
}
