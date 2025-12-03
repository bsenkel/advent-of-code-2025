advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let segments: Vec<&str> = input.split(',').collect();
    let mut add: u64 = 0;

    for segment in segments {
        let range: Vec<&str> = segment.split('-').collect();
        let first_id = range[0].parse::<u64>();
        let last_id = range[1].parse::<u64>();

        for i in first_id.unwrap_or_default()..=last_id.unwrap_or_default() {
            let i_cast = i.to_string();
            let i_size = i_cast.len();

            let (first, last) = i_cast.split_at(i_size / 2);
            if first == last {
                add += i;
            }
        }
    }

    Some(add)
}

pub fn part_two(input: &str) -> Option<u64> {
    let segments: Vec<&str> = input.split(',').collect();
    let mut sum: u64 = 0;

    for segment in segments {
        let range: Vec<&str> = segment.split('-').collect();
        let first_id = range[0].parse::<u64>();
        let last_id = range[1].parse::<u64>();

        for i in first_id.unwrap_or_default()..=last_id.unwrap_or_default() {
            let digits = i.to_string();
            let len = digits.len();

            for pattern_len in 1..=len / 2 {
                let pattern = &digits[0..pattern_len];
                let mut pos = 0;
                let mut count = 0;

                while pos + pattern_len <= len {
                    if &digits[pos..pos + pattern_len] == pattern {
                        count += 1;
                        pos += pattern_len;
                    } else {
                        break;
                    }
                }

                if count >= 2 && pos == len {
                    sum += i;
                    break;
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
