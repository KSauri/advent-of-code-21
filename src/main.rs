fn main() {
    println!(
        "day 1 part 1 solution: {}",
        day1::tally_increases(day1::get_measurements().unwrap())
    );
    println!(
        "day 1 part 2 solution: {}",
        day1::tally_3day_increases(day1::get_measurements().unwrap())
    );
}

mod day1;
