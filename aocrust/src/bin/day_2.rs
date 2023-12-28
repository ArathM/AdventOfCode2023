use std::fs::read_to_string;
use std::collections::HashMap;

pub enum Flag {
    Part1,
    Part2,
}

struct Game {
    id: i32,
    cube_game: HashMap<String, Vec<i32>>,
}

pub struct Day2 {
    input_file: String,
    flag: Flag,
}

impl Day2 {

    pub fn new(input_file: &str, flag: Flag) -> Day2 {
        Day2 {
            input_file: input_file.to_string(),
            flag
        }
    }
    
    fn read_lines(filename: &str) -> Vec<String> {
        let mut result = Vec::new();

        for line in read_to_string(filename).unwrap().lines() {
            result.push(line.to_string())
        }

        result
    }

    pub fn solve_problems(&self) {

        let lines: Vec<String> = Self::read_lines(&self.input_file);
        let mut games: Vec<Game> = Vec::new();
        let mut sum = 0;

        // Saving data for all games to then evaluate if they are possible
        for line in lines {

            //Initializing game
            let mut current_game = Game {
                id: 0,
                cube_game: HashMap::new(),
            };

            // Iterating over the lines and adding game id, and all games
            let cube_game = line.split(":").collect::<Vec<&str>>();
            let game_id = cube_game[0].split_whitespace().nth(1).unwrap().parse::<i32>().unwrap();

            current_game.id = game_id;

            for game in cube_game[1].split(";").collect::<Vec<&str>>() {
                for cube_info in game.split(",") {
                    let mut parts = cube_info.split_whitespace();
                    let number = parts.next().unwrap().parse::<i32>().unwrap();
                    let color = parts.next().unwrap().to_string();

                    current_game.cube_game.entry(color).or_insert(Vec::new()).push(number);
                }            
            }
            games.push(current_game);
        }

        match &self.flag {
            // Evaluating if the games are possible and summing their ids to sum if they are
            Flag::Part1 => {
                for game in games {
                    let mut possible = true;
                    for (color, numbers) in game.cube_game {
                        match color.as_str() {
                            "red" => {
                                if numbers.iter().any(|&x| x > 12) {
                                    possible = false;
                                }
                            },
                            "green" => {
                                if numbers.iter().any(|&x| x > 13) {
                                    possible = false;
                                }
                            },
                            "blue" => {
                                if numbers.iter().any(|&x| x > 14) {
                                    possible = false;
                                }
                            },
                            _ => {
                                    possible = false;
                            }
                        }
                    }
                    if possible {
                        sum += game.id;
                    }
                }
                println!("The sum of the ids of possible games is {}", sum);
            } Flag::Part2 => {
                for game in games {
                    let mut minimums : Vec<i32> = Vec::new();
                    minimums.push(*game.cube_game.get("red").unwrap().iter().max().unwrap());
                    minimums.push(*game.cube_game.get("green").unwrap().iter().max().unwrap());
                    minimums.push(*game.cube_game.get("blue").unwrap().iter().max().unwrap());

                    let power = minimums.iter().product::<i32>();

                    sum += power;
                }
                println!("The sum of the powers of the games is {}", sum);
            }
        }
    }
}