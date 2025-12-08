advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut fresh_ingredient_ranges = Vec::<&str>::new();
    let mut ingredient_ids = Vec::<u64>::new();
    let mut separator = false;

    for line in input.lines() {
        if !separator {
            fresh_ingredient_ranges.push(line);
        }
        if line.is_empty() {
            separator = true;
            continue;
        }

        // start here to process available ingredient IDs
        ingredient_ids.push(line.parse::<u64>().unwrap());
        // ...
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
