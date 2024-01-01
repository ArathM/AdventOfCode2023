use std::collections::HashMap;
use crate::{Flag, LineReader};

pub struct Day4 {
    input_file: String,
    flag: Flag,
}

impl LineReader for Day4 {}

impl Day4 {

    pub fn new(input_file: &str, flag: Flag) -> Day4 {
        Day4 {
            input_file: input_file.to_string(),
            flag
        }
    }

    pub fn solve_problems(&self) {

        let lines: Vec<String> = Self::read_lines(&self.input_file);
        // Hashmap that contains: (winning numbers, game_numbers) 
        // and another that contains (card_id, winning number count, scratchcard count)
        let mut numbers_game: HashMap<(u32,Vec<String>),Vec<String>> = HashMap::new();
        let mut winning_numbers_count: HashMap<u32,(u32,u32)> = HashMap::new();

        let mut count = 0;
        for line in lines {
            let data = line.split(":").collect::<Vec<&str>>()[1];
            let winning_numbers = data.split("|").collect::<Vec<&str>>()[0];
            let game_numbers = data.split("|").collect::<Vec<&str>>()[1];
            let winning_numbers_vec: Vec<String> = winning_numbers.split_whitespace().map(|s| s.to_string()).collect();
            let game_numbers_vec: Vec<String> = game_numbers.split_whitespace().map(|s| s.to_string()).collect();
            numbers_game.insert((count,winning_numbers_vec), game_numbers_vec);
            winning_numbers_count.insert(count,(1,0));
            count += 1;
        }

        let mut scratchcard_value = 0;

        for game in numbers_game {
            let mut winning_numbers = 0;
            for number in game.1 {
                if game.0.1.contains(&number) {
                    winning_numbers += 1;
                }
            }

            if winning_numbers >=1 {
                let card_value = u32::pow(2,winning_numbers-1);
                scratchcard_value += card_value;
            }

            winning_numbers_count.get_mut(&game.0.0).unwrap().1 += winning_numbers;
        }

        if matches!(self.flag, Flag::Part1) {
            println!("The value of the scratchcard is: {}", scratchcard_value);
        }
        
        else if matches!(self.flag, Flag::Part2) {

            for game in 0..winning_numbers_count.len() as u32 {
                for _ in 0..winning_numbers_count.get(&game).unwrap().0 {
                    for winning_card_count in 1..winning_numbers_count.get(&game).unwrap().1 as u32 +1 {
                        if let Some(value) = winning_numbers_count.get_mut(&(game+winning_card_count)) {
                            value.0 += 1;
                        }
                    }
                }
            }

           let scratchcards: u32 = winning_numbers_count.iter().map(|(_,v)| v.0).sum();
    
            println!("The total number of scratchcards is: {}", scratchcards);
        }
    }
}