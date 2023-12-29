#[path = "bin/day_1.rs"] mod day_1;
#[path = "bin/day_2.rs"] mod day_2;
#[path = "bin/day_3.rs"] mod day_3;
use day_1::Day1;
use day_2::Day2;
use day_3::Day3;
use day_1::Flag as D1Flag;
use day_2::Flag as D2Flag;
use day_3::Flag as D3Flag;

fn day_1() {
    let part1 = Day1::new("files/Day1.txt",D1Flag::Part1);
    let part2 = Day1::new("files/Day1.txt",D1Flag::Part2);
    part1.get_sum();
    part2.get_sum();
}

fn day_2() {
    let part1 = Day2::new("files/Day2.txt",D2Flag::Part1);
    let part2 = Day2::new("files/Day2.txt",D2Flag::Part2);
    part1.solve_problems();
    part2.solve_problems();
}

fn day_3() {
    let part1 = Day3::new("files/Day3.txt",D3Flag::Part1);
    let part2 = Day3::new("files/Day3.txt",D3Flag::Part2);
    part1.solve_problems();
}

fn main() {
   //day_1();
   //day_2();
   day_3();
}
