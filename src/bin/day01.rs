use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day01").unwrap();
    let solution1: u32 = input
        .lines()
        .map(|line| calibration_value1(line).unwrap())
        .sum();
    println!("solution for part one: {}", solution1);

    let solution2: u32 = input
        .lines()
        .map(|line| calibration_value2(line).unwrap())
        .sum();
    println!("solution for part two: {}", solution2);
}

fn calibration_value1(line: &str) -> Option<u32> {
    let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
    let first_digit = digits.first()?.to_digit(10)?;
    let last_digit = digits.last()?.to_digit(10)?;
    Some(first_digit * 10 + last_digit)
}

fn calibration_value2(line: &str) -> Option<u32> {
    let digit_values = (1..=9).map(|d| (d.to_string(), d));
    let spelled_values = vec![
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ];

    // (digit_value, index)
    let mut leftmost_digit = (0, line.len());
    let mut rightmost_digit = (0, 0);

    for (digit_spelling, digit_value) in digit_values.chain(spelled_values) {
        for (index, _) in line.match_indices(&digit_spelling) {
            if index <= leftmost_digit.1 {
                leftmost_digit = (digit_value, index);
            }
            if index >= rightmost_digit.1 {
                rightmost_digit = (digit_value, index);
            }
        }
    }

    if leftmost_digit.0 == 0 || rightmost_digit.0 == 0 {
        return None
    }

    let value = leftmost_digit.0 * 10 + rightmost_digit.0;
    Some(value)
}

#[test]
fn test_day_01_examples() {
    assert_eq!(calibration_value1("1abc2"), Some(12));
    assert_eq!(calibration_value1("pqr3stu8vwx"), Some(38));
    assert_eq!(calibration_value1("a1b2c3d4e5f"), Some(15));
    assert_eq!(calibration_value1("treb7uchet"), Some(77));

    assert_eq!(calibration_value2("1abc2"), Some(12));
    assert_eq!(calibration_value2("pqr3stu8vwx"), Some(38));
    assert_eq!(calibration_value2("a1b2c3d4e5f"), Some(15));
    assert_eq!(calibration_value2("treb7uchet"), Some(77));

    assert_eq!(calibration_value2("two1nine"), Some(29));
    assert_eq!(calibration_value2("eightwothree"), Some(83));
    assert_eq!(calibration_value2("abcone2threexyz"), Some(13));
    assert_eq!(calibration_value2("xtwone3four"), Some(24));
    assert_eq!(calibration_value2("4nineeightseven2"), Some(42));
    assert_eq!(calibration_value2("zoneight234"), Some(14));
    assert_eq!(calibration_value2("7pqrstsixteen"), Some(76));
}
