use std::fs::read_to_string;
use fancy_regex::Regex;
use std::collections::HashMap;

pub enum Flag {
    Part1,
    Part2,
}

pub struct Day1 {
    input_file: String,
    flag: Flag,
}

impl Day1 {

    pub fn new(input_file: &str, flag: Flag) -> Day1 {
        Day1 {
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

    pub fn get_sum(&self) {

        let lines: Vec<String> = Self::read_lines(&self.input_file);
        let mut sum = 0;

        for line in lines {
            // One string to iterate over the line, another to store only digits
            let line_string= line.chars().collect::<String>();
            let mut digits_and_words_str = String::new();

            //Optional matching words to numbers in part two
            match &self.flag {
                // Iterate over the captures and add to a new string, alternatively, 
                // we could have removed the non digit characters from the string
                Flag::Part1 => {

                    for (_, c) in line_string.char_indices() {
                        if c.is_digit(10) {
                            digits_and_words_str.push(c);
                        }
                    }
                },
                Flag::Part2 => {

                    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();

                    // Create a HashMap
                    let map: HashMap<&str, i32> = [
                        ("one", 1),
                        ("two", 2),
                        ("three", 3),
                        ("four", 4),
                        ("five", 5),
                        ("six", 6),
                        ("seven", 7),
                        ("eight", 8),
                        ("nine", 9),
                    ].iter().cloned().collect();

                    for (i, c) in line_string.char_indices() {
                        if !c.is_digit(10) {
                            if let Ok(Some(mat)) = re.find(&line_string[i..])  {
                                // Add to the string only if the match is at the current position of the string
                                // to avoid repeated regex matches
                                if mat.start() == 0 {
                                    digits_and_words_str.push_str(&map.get(mat.as_str()).unwrap().to_string());
                                }
                            } 
                        } else if c.is_digit(10) {
                            digits_and_words_str.push(c);
                        }
                    }
                },
            }
            // If the string is not empty, get the first and last digits and add them to the sum
            // If there is only one digit, multiply it by 11 (repeat the same digit twicce) and add it to the sum

            if !digits_and_words_str.is_empty() {
                if digits_and_words_str.len() > 1 {
                    let first_digit = digits_and_words_str.chars().next().unwrap();
                    let last_digit = digits_and_words_str.chars().last().unwrap();
                    let full_digit = format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
                    sum += full_digit;
                } else {
                    let number = digits_and_words_str.parse::<i32>().unwrap();
                    sum += number * 11;
                }
            }
        }
        println!("The sum of the first and last digits on each line is: {}", sum);
    }
} 
