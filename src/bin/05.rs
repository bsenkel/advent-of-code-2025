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

    // TODO: sort ranges and merge
    fresh_ingredient_ranges.sort();

    println!("{:?}", fresh_ingredient_ranges);
    println!("Lines: {}", fresh_ingredient_ranges.len());

    println!("{:?}", ingredient_ids);
    println!("Lines: {}", ingredient_ids.len());

    for id in ingredient_ids {
        let mut found = false;
        for range in fresh_ingredient_ranges.iter() {
            for i in range.start..=range.end {
                if i == id {
                    fresh_count += 1;
                    found = true;
                    break;
                }
            }
            if found {
                break;
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
