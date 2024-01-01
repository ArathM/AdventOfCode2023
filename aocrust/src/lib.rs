use std::fs::read_to_string;
pub mod problems;

pub enum Flag {
    Part1,
    Part2,
}

pub trait LineReader {

    fn read_lines(filename: &str) -> Vec<String> {
        let mut result = Vec::new();

        for line in read_to_string(filename).unwrap().lines() {
            result.push(line.to_string())
        }

        result
    }

}