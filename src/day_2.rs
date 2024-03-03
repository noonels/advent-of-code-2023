use std::{cmp::max, fmt::Display};

pub struct Game {
    pub id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    pub fn get_power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Game {id}: ({red}, {green}, {blue})",
            id = self.id,
            red = self.red,
            green = self.green,
            blue = self.blue
        )
    }
}

impl From<&str> for Game {
    fn from(game_line: &str) -> Self {
        let mut line_parts: Vec<&str> = game_line.split(':').collect();

        // `pop` pulls from the back, so we go back to front here
        let games = line_parts.pop().expect("could not find games list");
        let game_header = line_parts.pop().expect("could not find game header");
        let game_id = game_header.split(' ').collect::<Vec<&str>>()[1];

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for game in games.split("; ") {
            for color in game.split(", ") {
                let color = color.trim();
                if let [val, color_str, ..] = color.split(" ").collect::<Vec<&str>>()[..] {
                    let val = val
                        .parse::<i32>()
                        .expect(format!("Unable to parse {val} as i32").as_str());
                    match color_str {
                        "red" => red = max(red, val),
                        "green" => green = max(green, val),
                        "blue" => blue = max(blue, val),
                        _ => println!("Bad color {color_str}"),
                    }
                }
            }
        }

        Game {
            id: game_id.parse().expect("Game id could not be parsed"),
            red,
            green,
            blue,
        }
    }
}

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;
pub fn tabulate_games(content: &str, filter: Option<bool>) -> Result<Vec<Game>, std::io::Error> {
    Ok(content
        .lines()
        .map(|line| Game::from(line))
        .filter(|game| match filter {
            Some(true) => game.red <= MAX_RED && game.green <= MAX_GREEN && game.blue <= MAX_BLUE,
            _ => true,
        })
        .collect())
}
