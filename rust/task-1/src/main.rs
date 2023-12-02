use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = file_path_from_args(&args);
    let content = fs::read_to_string(file_path).expect(&*format!("{} does not exist", file_path));
    let result = calculate_result(&content);
    println!("Result is {}", result)
}

fn file_path_from_args(args: &[String]) -> &str {
    args.get(1).map_or("./data.txt", |s| &s)
}

fn calculate_result(content: &str) -> i32 {
    content.lines().map(process_line).sum()
}

fn process_line(line: &str) -> i32 {
    let digits: Vec<i32> = line.chars().filter(|c| c.is_numeric()).map(|c| c as i32 - 0x30).collect();
    let digest_str = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());

    return digest_str.parse::<i32>().unwrap();
}
