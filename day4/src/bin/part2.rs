use std::fs;

const FILE: &str = "input";

fn main() {
    let content = fs::read_to_string(FILE).unwrap();
    let input: Vec<&str> = content.lines().collect();
    let res = solve(input.clone(), input.len());
    println!("{}", res);
}

fn solve(arr: Vec<&str>, n: usize) -> i32 {
    let mut nums = vec![0; n];

    arr.iter().enumerate().for_each(|(i, &f)| {
        let w = winning(f.split(':').last().unwrap().split('|').collect());
        nums[i] += 1;
        for idx in i + 1..i + 1 + w as usize {
            nums[idx] += nums[i];
        }
    });
    for (i, f) in nums.iter().enumerate() {
        println!("{}: {}", i, *f);
    }
    nums.iter().sum()
}

fn winning(arr: Vec<&str>) -> i32 {
    let a: Vec<&str> = arr.last().unwrap().split_whitespace().collect();

    let mut i = 0;
    arr.first().unwrap().split_whitespace().for_each(|f| {
        if a.contains(&f) {
            i += 1
        }
    });

    i
}
