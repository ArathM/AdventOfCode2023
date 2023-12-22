use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

pub fn get_sum() {
    let lines: Vec<String> = read_lines("../files/Day1.txt");
    for line in lines {
        let mut _count = 0;
        for c in line.chars() {
            {
                _count += 1;
                println!("{}", c)}
        }
        let first = line.parse::<i32>().unwrap();
        let last = line.parse::<i32>().unwrap();
        println!("{} + {} = {}", first, last, first + last);
    }
}
