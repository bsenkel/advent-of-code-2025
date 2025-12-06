use std::u32;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut max_joltage: u64 = 0;

    for line in input.lines().filter_map(|l| {
        if l.is_empty() {
            return None;
        }
        Some(l)
    }) {
        let digits = line;
        let len = digits.len();
        let mut digit1 = u32::MIN;
        let mut digit1_pos = 0;
        let mut digit2 = u32::MIN;
        let mut digit2_pos = 0;

        for (index, digit) in digits.chars().enumerate() {
            let number = digit.to_digit(10).unwrap();
            if number > digit1 {
                digit1 = number;
                digit1_pos = index;
            }
        }

        // iterate to the right side
        if digit1_pos < len - 1 {
            for (index, digit) in digits[digit1_pos + 1..len].chars().enumerate() {
                let number = digit.to_digit(10).unwrap();
                if number > digit2 {
                    digit2 = number;
                    digit2_pos = index + digit1_pos + 1;
                }
            }
        } else {
            // iterate to the left side
            for (index, digit) in digits[0..digit1_pos].chars().rev().enumerate() {
                let number = digit.to_digit(10).unwrap();
                if number > digit2 {
                    digit2 = number;
                    digit2_pos = digit1_pos - 1 - index;
                }
            }
        }

        let result;
        if digit1_pos < digit2_pos {
            result = format!("{}{}", digit1, digit2);
        } else {
            result = format!("{}{}", digit2, digit1);
        }

        let joltage: u64 = result.parse().unwrap();
        max_joltage += joltage;
    }

    Some(max_joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut max_joltage: u64 = 0;

    for line in input.lines().filter_map(|l| {
        if l.is_empty() {
            return None;
        }
        Some(l)
    }) {
        let digits = line;
        let len = digits.len();

        let mut start = 0;
        let mut values: Vec<u32> = Vec::new();

        for i in (0..12).rev() {
            let mut max = u32::MIN;
            let mut max_pos = start;

            let end = len - i;

            if start >= end {
                break;
            }

            for (index, digit) in digits[start..end].chars().enumerate() {
                let number = digit.to_digit(10).unwrap();
                if number > max {
                    max = number;
                    max_pos = start + index;
                }
            }

            values.push(max);
            start = max_pos + 1;
        }

        let result: String = values.iter().map(|v| v.to_string()).collect();

        let joltage: u64 = result.parse().unwrap();
        max_joltage += joltage;
    }

    Some(max_joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
