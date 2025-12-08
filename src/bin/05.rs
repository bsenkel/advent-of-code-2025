advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut fresh_ingredient_ranges = Vec::<&str>::new();

    for line in input.lines() {
        if !line.is_empty() {
            fresh_ingredient_ranges.push(line);
        } else {
            // start here to process available ingredient IDs
            // ...
        }
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
