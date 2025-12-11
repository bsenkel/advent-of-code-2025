advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    // read input data
    // iterate over each row until no numbers are left
    // read symbols for the operation, perform the operation on the problem while iterating
    // save the current problem result
    // move to next problem, separated by full column of only spaces
    // repeat
    // grand total = sum of all answers of each problem

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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
