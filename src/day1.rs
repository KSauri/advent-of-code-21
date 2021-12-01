use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn get_measurements() -> Result<Vec<u32>> {
    let mut file =
        File::open("/Users/kylesauri/Development/advent-of-code-21/test_input/day1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut result = Vec::new();
    for measurements in contents.lines() {
        result.push(measurements.parse::<u32>().unwrap());
    }
    Ok(result)
}

pub fn tally_increases(measurements: Vec<u32>) -> u32 {
    let mut result = 0;
    measurements
        .iter()
        .enumerate()
        .for_each(|(idx, measurement)| {
            if idx != 0 {
                if measurement > &measurements[idx - 1] {
                    result += 1;
                }
            }
        });

    result
}

pub fn tally_3day_increases(measurements: Vec<u32>) -> u32 {
    let mut result = 0;
    for idx in 2..(measurements.len() - 1) {
        let previous_3day_window_sum: u32 = measurements[idx - 2..idx + 1].iter().sum();
        let current_3day_window_sum: u32 = measurements[idx - 1..idx + 2].iter().sum();
        if current_3day_window_sum > previous_3day_window_sum {
            result += 1;
        }
    }
    result
}

mod test {
    use super::*;

    #[test]
    fn read_input() {
        let input = get_measurements().unwrap();
        assert_eq!(input.len(), 2000)
    }

    #[test]
    fn tally_increases_non_zero() {
        let input = get_measurements().unwrap();
        assert!(tally_increases(input) > 0)
    }

    #[test]
    fn tally_3day() {
        let input = vec![0, 0, 0, 1, 1, 1, 0, 0, 0, 2, 2, 2];
        assert_eq!(tally_3day_increases(input), 6);
    }
}
