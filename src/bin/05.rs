advent_of_code::solution!(5);

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err("Invalid format".into());
        }

        Ok(Range {
            start: parts[0].parse()?,
            end: parts[1].parse()?,
        })
    }
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort();

    let mut merged = vec![ranges[0]];

    for current in ranges.iter().skip(1) {
        let last = merged.last_mut().unwrap();

        // merge, if they overlap or are neighbors
        if current.start <= last.end + 1 {
            last.end = last.end.max(current.end);
        } else {
            // or add new range
            merged.push(*current);
        }
    }

    merged
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut fresh_ingredient_ranges = vec![];
    let mut ingredient_ids = Vec::<u64>::new();
    let mut separator = false;
    let mut fresh_count = 0;

    for line in input.lines() {
        if line.is_empty() {
            separator = true;
            continue;
        }
        if !separator {
            if let Ok(range) = Range::from_str(line) {
                fresh_ingredient_ranges.push(range);
            }
        }

        if let Ok(num) = line.parse::<u64>() {
            ingredient_ids.push(num);
        }
    }

    let merged = merge_ranges(fresh_ingredient_ranges);

    for id in ingredient_ids {
        for range in merged.iter() {
            if id >= range.start && id <= range.end {
                fresh_count += 1;
            }
        }
    }

    Some(fresh_count)
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
