use aocrust::Flag;
use aocrust::problems::day_1::Day1;
use aocrust::problems::day_2::Day2;
use aocrust::problems::day_3::Day3;
use aocrust::problems::day_4::Day4;

fn day_1() {
    let part1 = Day1::new("files/Day1.txt",Flag::Part1);
    let part2 = Day1::new("files/Day1.txt",Flag::Part2);
    part1.solve_problems();
    part2.solve_problems();
}

fn day_2() {
    let part1 = Day2::new("files/Day2.txt",Flag::Part1);
    let part2 = Day2::new("files/Day2.txt",Flag::Part2);
    part1.solve_problems();
    part2.solve_problems();
}

fn day_3() {
    let part1 = Day3::new("files/Day3.txt",Flag::Part1);
    let part2 = Day3::new("files/Day3.txt",Flag::Part2);
    part1.solve_problems();
    part2.solve_problems();
}

fn day_4() {
    let part1 = Day4::new("files/Day4.txt",Flag::Part1);
    let part2 = Day4::new("files/Day4.txt",Flag::Part2);
    part1.solve_problems();
    part2.solve_problems();
}

fn main() {
   day_1();
   day_2();
   day_3();
   day_4();
}
