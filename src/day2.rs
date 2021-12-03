use itertools::Itertools;
// use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn part_1(direction: &str, magnitude: u32, mut position: (u32, u32, u32)) -> (u32, u32, u32) {
    match direction {
        "forward" => position.0 += magnitude,
        "up" => position.1 -= magnitude,
        "down" => position.1 += magnitude,
        _ => {}
    }
    position
}

pub fn part_2(direction: &str, magnitude: u32, mut position: (u32, u32, u32)) -> (u32, u32, u32) {
    match direction {
        "forward" => {
            position.0 += magnitude;
            position.1 += position.2 * magnitude;
        }
        "up" => position.2 -= magnitude,
        "down" => position.2 += magnitude,
        _ => {}
    }
    position
}

pub fn get_position(
    filename: &str,
    position_delta: fn(&str, u32, (u32, u32, u32)) -> (u32, u32, u32),
) -> Result<u32> {
    let mut file = File::open(filename)?;
    let mut commands = String::new();
    file.read_to_string(&mut commands)?;
    let mut position = (0, 0, 0);
    let _ = commands
        .lines()
        .map(|line| {
            line.split(' ')
                .collect_tuple()
                .map(|(direction, magnitude)| (direction, magnitude.parse::<u32>().unwrap()))
                .map(|(direction, magnitude)| {
                    position = position_delta(direction, magnitude, position)
                })
        })
        .collect::<Vec<_>>();
    Ok(position.0 * position.1)
}

mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let output = get_position(
            "/Users/kylesauri/Development/advent-of-code-21/test_input/day2_simple.txt",
            part_1,
        )
        .unwrap();
        assert_eq!(output, 150)
    }

    #[test]
    fn test_part_2() {
        let output = get_position(
            "/Users/kylesauri/Development/advent-of-code-21/test_input/day2_simple.txt",
            part_2,
        )
        .unwrap();
        assert_eq!(output, 900)
    }
}
