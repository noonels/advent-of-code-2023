use regex::Regex;
use std::num::ParseIntError;

pub fn parse_lines(content: &str) -> Result<Vec<i32>, ParseIntError> {
    content
        .lines()
        .map(|line| parse_line(&parse_nums_in_line(line)))
        .collect()
}

fn parse_line(line: &str) -> Result<i32, ParseIntError> {
    let mut iter = line.chars().filter(|c| c.is_numeric()); // filter redundant for part two
    let first_num = iter.next().unwrap_or('0');
    let last_num = iter.last().unwrap_or(first_num);
    let result = format!("{}{}", first_num, last_num).parse::<i32>()?;
    Ok(result)
}

fn parse_nums_in_line(line: &str) -> String {
    let re = Regex::new(r"(oneight|threeight|fiveight|nineight|twone|sevenine|eightwo|eighthree|one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    let mut results = String::new();
    for (_, [digit]) in re.captures_iter(line).map(|c| c.extract()) {
        let new_digit = match digit {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            "oneight" => "18",
            "threeight" => "38",
            "fiveight" => "58",
            "nineight" => "98",
            "twone" => "21",
            "sevenine" => "79",
            "eightwo" => "82",
            "eighthree" => "83",
            _ => digit,
        };
        results.push_str(new_digit);
    }
    results
}
