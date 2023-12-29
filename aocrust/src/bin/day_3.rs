use std::fs::read_to_string;
use std::collections::HashMap;

pub enum Flag {
    Part1,
    Part2,
}

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

impl Schematic {
    
    // Function to check if a number contains adjacent symbols
    // by checking the starting position up to the length of the number
    pub fn contains_adjacent_symbols(&mut self, x: usize, y: usize, len: usize) {
        let columns = self.matrix[0].len();

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

        // Adding numbers to the number hashmap by position
        // Also, replacing symbols that aren't dots to a single value (*)
        // and storing it in matrix format

        let COLUMNS = lines[0].len();
        let v = vec!['.'; COLUMNS+2];

        schematic.matrix.push(v.clone()).clone();

        let mut x = 1;
        for line in lines {
            let mut row = Vec::new();
            let mut current_number = String::new();
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
                    if usize::from(y) == COLUMNS {
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

            schematic.matrix.push(row.clone()).clone();
            x+=1;
        }
        schematic.matrix.push(v.clone()).clone();

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
        println!("The sum of the numbers that contain adjacent symbols is {}", sum);
    }
}