fn main() {
    println!(
        "day 1 part 1 solution: {}",
        day1::tally_increases(day1::get_measurements().unwrap())
    );
    println!(
        "day 1 part 2 solution: {}",
        day1::tally_3day_increases(day1::get_measurements().unwrap())
    );

    println!(
        "day 2 part 1 solution: {:?}",
        day2::get_position(
            "/Users/kylesauri/Development/advent-of-code-21/test_input/day2.txt",
            day2::part_1
        )
        .unwrap()
    );

    println!(
        "day 2 part 2 solution: {:?}",
        day2::get_position(
            "/Users/kylesauri/Development/advent-of-code-21/test_input/day2.txt",
            day2::part_2
        )
        .unwrap()
    );
}

mod day1;
mod day2;
