use std::fs;

const FILE: &str = "prueba";
fn main() {
    let content = fs::read_to_string(FILE).unwrap();
    let input: Vec<&str> = content.split('\n').collect();
    let len = input.len();
    let anchura = input.first().unwrap().len();
}
