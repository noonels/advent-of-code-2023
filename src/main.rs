fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[2];
    let content = std::fs::read_to_string(file_path).expect("Should be able to read input file");
    let result: i32 = content.lines().map(|line| {
        let mut iter = line.chars().filter(|c| c.is_numeric());
        let first_num = iter.next().expect("Should have first num");
        let last_num = iter.last().unwrap_or(first_num);
        format!("{}{}", first_num, last_num)
    }).map(|line| {
        line.parse::<i32>()
            .expect(format!("No valid number found in line {}", line).as_str())
    }).sum();

    println!("{:?}", result)
}
