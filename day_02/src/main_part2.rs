use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn read_file_line_by_line(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

struct GameAnalyzer {
    _index: i32,
    max_blues: i32,
    max_reds: i32,
    max_greens: i32,
}

fn get_game_power(game_analyzer: GameAnalyzer) -> i32 {
    return game_analyzer.max_blues * game_analyzer.max_greens * game_analyzer.max_reds;
}

fn analyze_games(round_data: &str, game_analyzer: &mut GameAnalyzer) {
    let cube_counts: Vec<&str> = round_data.split(',').collect();

    println!("Round Data: {}", round_data);

    for cube_data in cube_counts {
        let how_many_cubes = cube_data.trim().split(' ').nth(0).unwrap();
        let cube_color = cube_data.trim().split(' ').nth(1).unwrap();

        match cube_color {
            "red" => {
                if how_many_cubes.parse::<i32>().unwrap() > game_analyzer.max_reds {
                    game_analyzer.max_reds = how_many_cubes.parse::<i32>().unwrap()
                }
            }
            "green" => {
                if how_many_cubes.parse::<i32>().unwrap() > game_analyzer.max_greens {
                    game_analyzer.max_greens = how_many_cubes.parse::<i32>().unwrap()
                }
            }
            "blue" => {
                if how_many_cubes.parse::<i32>().unwrap() > game_analyzer.max_blues {
                    game_analyzer.max_blues = how_many_cubes.parse::<i32>().unwrap()
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut sum_of_cube_powers = 0;

    let file_path = &args[1];

    let lines = read_file_line_by_line(&file_path);

    for line in lines {
        let game_data = line.split(':').nth(0).unwrap();
        let cube_details = line.split(':').nth(1).unwrap();

        let game_index = game_data.split(" ").nth(1).unwrap().parse::<i32>().unwrap();

        let mut game_analyzer = GameAnalyzer {
            _index: game_index,
            max_reds: 0,
            max_greens: 0,
            max_blues: 0,
        };

        let games: Vec<&str> = cube_details.split(';').collect();

        for game in games {
            analyze_games(game, &mut game_analyzer);
        }

        sum_of_cube_powers += get_game_power(game_analyzer)
    }

    println!("Sum of Game Powers: {}", sum_of_cube_powers);
}
