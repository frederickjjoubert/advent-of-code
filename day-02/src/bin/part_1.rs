use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    game_number: usize,
    sub_games: Vec<SubGame>,
}

#[derive(Debug)]
struct SubGame {
    colors: HashMap<String, i32>,
}

impl Game {
    fn new(game_number: usize, sub_games: Vec<SubGame>) -> Game {
        Game { game_number, sub_games }
    }
}

impl SubGame {
    fn new(colors: HashMap<String, i32>) -> SubGame {
        SubGame { colors }
    }

    fn from_str(s: &str) -> SubGame {
        let mut colors = HashMap::new();
        s.split(',').for_each(|color_count| {
            let parts: Vec<&str> = color_count.trim().split_whitespace().collect();
            if parts.len() == 2 {
                let count = parts[0].parse::<i32>().unwrap_or(0);
                let color = parts[1].to_string();
                colors.insert(color, count);
            }
        });
        SubGame::new(colors)
    }
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let sub_games = line.split(';')
                .map(|sub_game_str| SubGame::from_str(sub_game_str))
                .collect();
            Game::new(index + 1, sub_games)
        })
        .collect()
}

fn main() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    let red: i32 = 12;
    let green: i32 = 13;
    let blue: i32 = 14;

    let mut total: u32 = 0;

    let games = parse_games(input);
    for game in games {
        let mut is_valid_game = true;
        for sub_game in &game.sub_games {
            

            if sub_game.colors.contains_key("red") {
                let sub_game_red = sub_game.colors["red"];
                if sub_game_red > red {
                    is_valid_game = false;
                }
            }

            if sub_game.colors.contains_key("green") {
                let sub_game_green = sub_game.colors["green"];
                if sub_game_green > green {
                    is_valid_game = false;
                }
            }

            if sub_game.colors.contains_key("blue") {
                let sub_game_blue = sub_game.colors["blue"];
                if sub_game_blue > blue {
                    is_valid_game = false;
                }
            }

            

        }
        if is_valid_game {
            total += game.game_number as u32;
        }
        println!("{:?}", game);
    }
    println!("{:?}", total);
}
