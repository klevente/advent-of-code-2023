use advent_of_code_2023::read_file_lines;
use phf::phf_map;

static DIGITS: phf::Map<&'static str, u32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

fn calculate_calibration_value_digits_only(line: &str) -> u32 {
    let mut first_digit = None;
    let mut last_digit = None;

    for c in line.chars() {
        if c.is_digit(10) {
            let digit = c.to_digit(10).unwrap();
            if first_digit.is_none() {
                first_digit = Some(digit);
            }
            last_digit = Some(digit);
        }
    }

    first_digit.unwrap() * 10 + last_digit.unwrap()
}

fn calculate_sum_of_calibration_values_digits_only(lines: &Vec<String>) -> u32 {
    lines.iter().fold(0, |acc, next| {
        acc + calculate_calibration_value_digits_only(next)
    })
}

fn calculate_calibration_values_with_words(line: &str) -> u32 {
    let mut first_digit = None;
    let mut last_digit = None;

    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            let digit = c.to_digit(10).unwrap();
            if first_digit.is_none() {
                first_digit = Some(digit);
            }
            last_digit = Some(digit);
        } else {
            let (_, split) = line.split_at(i);
            for (k, v) in &DIGITS {
                if split.starts_with(k) {
                    if first_digit.is_none() {
                        first_digit = Some(*v);
                    }
                    last_digit = Some(*v);
                }
            }
        }
    }

    first_digit.unwrap() * 10 + last_digit.unwrap()
}

fn calculate_sum_of_calibration_values_with_words(lines: &Vec<String>) -> u32 {
    lines.iter().fold(0, |acc, next| {
        acc + calculate_calibration_values_with_words(next)
    })
}

fn main() {
    let input = read_file_lines("input/day1.txt");

    let sum_of_calibration_values_digits_only =
        calculate_sum_of_calibration_values_digits_only(&input);
    println!("The sum of calibration values considering only digits is: {sum_of_calibration_values_digits_only}");

    let sum_of_calibration_values_with_words =
        calculate_sum_of_calibration_values_with_words(&input);
    println!("The sum of calibration values considering words as well is: {sum_of_calibration_values_with_words}");
}
