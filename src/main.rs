mod day_1;
mod day_2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: advent_of_code_2018 <day> <input_file>");
        return;
    }

    let day_arg = &args[1];
    let file_path = &args[2];
    let content = std::fs::read_to_string(file_path).expect("Should be able to read input file");

    match day_arg.as_str() {
        "day1" => {
            let result: Result<Vec<i32>, _> = day_1::parse_lines(content.as_str());
            match result {
                Ok(v) => {
                    let res: i32 = v.iter().sum();
                    println!("{}", res)
                }
                Err(_) => println!("Error while finding result"),
            }
        }

        "day2" => {
            let games = day_2::tabulate_games(content.as_str(), Some(true));
            match games {
                Ok(v) => {
                    let result: i32 = v.iter().map(|g| g.id).sum();
                    println!("result returned: {result}")
                }
                Err(_) => println!("Error while finding result"),
            }
        }
        "day2.1" => {
            let games = day_2::tabulate_games(content.as_str(), None);
            match games {
                Ok(gs) => {
                    let result: i32 = gs.iter().map(|g| g.get_power()).sum();
                    println!("{result}");
                }
                Err(why) => println!("Error while doing challenge: {why}"),
            }
        }

        _ => println!("No action for day '{day_arg}'"),
    }
}
