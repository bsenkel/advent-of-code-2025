advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    // notes:
    // * in each line of input (= a battery bank) exactly two batteries need to be turned on
    // * battery bank joltage = digits of the turned on batteries, max. 2 digits, strict order, position can not be changed
    // * iterate through each line, find the largest single digit, check right and left neighbors‚ if available...
    // * total output joltage = sum of maximum joltage from each bank

    for line in input.lines().filter_map(|l| {
        if l.is_empty() {
            return None;
        }
        Some(l)
    }) {
        let digits = line;
        let mut max_digit = 0;
        let mut max_pos = 0;
        let mut count = 0;

        for digit in digits.chars() {
            let number = digit.to_digit(10).unwrap();
            if count == 0 {
                max_digit = number;
            }
            if number > max_digit {
                max_digit = number;
                max_pos = count;
            }
            count += 1;
        }

        println!("Found max val: {} at position {}", max_digit, max_pos);
    }

    Some(0)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
