use std::fs::read_to_string;
use std::collections::HashMap;
use crate::Flag;

pub struct Day3 {
    input_file: String,
    flag: Flag,
}

// Struct to hold all data, as well as the starting position
// of the numbers in the matrix, the length of the number, the number itself and whether
// or not it contains adjacent symbols
struct Schematic {
    matrix : Vec<Vec<char>>,
    numbers : HashMap<(u8, u8, u8), (u32,bool)>
}

// Struct that holds the position of a cog, as well as its part numbers if any.
// We only need the coordinates since cogs are a single character
struct CogSchematic {
    cogs : HashMap<(u8, u8), Vec<i32>>
}

impl Schematic {
    
    // Function to check if a number contains adjacent symbols
    // by checking the starting position up to the length of the number
    pub fn contains_adjacent_symbols(&mut self, x: usize, y: usize, len: usize) {

        for i in 0..len {
            if matches!(self.matrix[x-1][y-1+i], '*' | '0'..='9')
            || matches!(self.matrix[x-1][y+i], '*' | '0'..='9')
            || matches!(self.matrix[x-1][y+1+i], '*' | '0'..='9')
            || matches!(self.matrix[x][y-1+i], '*' )
            || matches!(self.matrix[x][y+1+i], '*' )
            || matches!(self.matrix[x+1][y-1+i], '*' | '0'..='9')
            || matches!(self.matrix[x+1][y+i], '*' | '0'..='9')
            || matches!(self.matrix[x+1][y+1+i], '*' | '0'..='9') {
                self.numbers.get_mut(&(x as u8, y as u8, len as u8)).unwrap().1 = true;
            }
        }
    }

}

impl CogSchematic {

    // Given a CogSchematic object with the position of the cogs,
    // fills the part numbers around each cog
    pub fn get_cog_ratios(&mut self, part_numbers: HashMap<(u8, u8, u8), (u32,bool)>) {

        let cog_keys = self.cogs.keys().cloned().collect::<Vec<_>>();
        for cog in cog_keys {
            let x = cog.0;
            let y = cog.1;

            // Checking the 8 positions around the cog
            for part_number in &part_numbers {
                if part_number.1 .1 {
                    let part_number_x = part_number.0.0;
                    let part_number_y = part_number.0.1;
                    let part_number_len = part_number.0.2;
                    let mut added = false;

                    for i in 0..part_number_len {
                        if (part_number_x == x-1 && part_number_y+i == y-1
                        || part_number_x == x-1 && part_number_y+i == y
                        || part_number_x == x-1 && part_number_y+i == y+1
                        || part_number_x == x && part_number_y+i == y-1
                        || part_number_x == x && part_number_y+i == y+1
                        || part_number_x == x+1 && part_number_y+i == y-1
                        || part_number_x == x+1 && part_number_y+i == y
                        || part_number_x == x+1 && part_number_y+i == y+1) 
                        && !added{
                            self.cogs.get_mut(&cog).unwrap().push(part_number.1.0 as i32); 
                            added = true;  
                        }
                    }
                }
            }
        }
    }
}

impl Day3 {

    pub fn new(input_file: &str, flag: Flag) -> Day3 {
        Day3 {
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
        let mut schematic = Schematic {
            matrix: Vec::new(),
            numbers: HashMap::new(),
        };
        let mut cogs = CogSchematic {
            cogs: HashMap::new(),
        };

        // Adding numbers to the number hashmap by position
        // Also, replacing symbols that aren't dots to a single value (*)
        // and storing it in matrix format

        let columns = lines[0].len();
        let v = vec!['.'; columns+2];

        let _ = schematic.matrix.push(v.clone()).clone();

        let mut x = 1;
        for line in lines {
            let mut row = Vec::new();
            let mut current_number = String::new();

            let mut y=1;
            //First storing only cogs positions
            for c in line.chars() {
                if c == '*' {
                    cogs.cogs.insert((x,y), Vec::new());
                }
                y+=1;
            }

            let mut y=1;
            // Adding a dot to the beginning and end of each row
            // and an extra row at the beginning and end, to avoid out of bounds errors
            row.push('.');
            for c in line.chars() {
                if c != '.' && !c.is_digit(10) {
                    if !current_number.is_empty() {
                        schematic.numbers.insert((x,y-current_number.len() as u8, current_number.len() as u8), (current_number.parse::<u32>().unwrap(), false));
                        current_number.clear();
                    }
                    row.push('*');
                } else if c.is_digit(10) {
                    current_number.push(c);
                    row.push(c);
                    if usize::from(y) == columns {
                        schematic.numbers.insert((x,y-current_number.len() as u8+1, current_number.len() as u8), (current_number.parse::<u32>().unwrap(), false));
                        current_number.clear();
                    }
                }
                else{
                    if !current_number.is_empty() {
                        schematic.numbers.insert((x,y-current_number.len() as u8, current_number.len() as u8), (current_number.parse::<u32>().unwrap(), false));
                        current_number.clear();
                    }
                    row.push('.');
                }
                y+=1;
            }
            row.push('.');

            let _ = schematic.matrix.push(row.clone()).clone();
            x+=1;
        }
        let _ = schematic.matrix.push(v.clone()).clone();

        // Detecting the numbers that contain adjacent symbols

        // Finally marking the numbers that contain adjacent symbols and summing their values
        let mut sum = 0;
        let keys = schematic.numbers.keys().cloned().collect::<Vec<_>>();
        for (x, y, len) in keys {
            schematic.contains_adjacent_symbols(x as usize,y as usize,len as usize);
        }
        let values = schematic.numbers.values().cloned().collect::<Vec<_>>();

        for (number, contains_adjacent_symbols) in values {
            if contains_adjacent_symbols {
                sum += number;
            }
        }

        if matches!(self.flag, Flag::Part1) {
            println!("The sum of the numbers that contain adjacent symbols is {}", sum);
        }
        else if matches!(self.flag, Flag::Part2) {
            // Getting the part numbers around each cog
            cogs.get_cog_ratios(schematic.numbers);

            // Getting the gear_ratios for each cog and its sum
            let mut sum = 0;
            let values = cogs.cogs.values().cloned().collect::<Vec<_>>();
            for cog in values {
                if cog.len() >= 2 {
                    sum += cog.iter().product::<i32>();
                }
            }
            println!("The sum of the gear_ratios of each cog is {}", sum);
        }
    }
}