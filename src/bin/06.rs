use std::num;

advent_of_code::solution!(6);

enum Operation {
    Add,
    Multiply,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    let mut columns: Vec<Vec<&str>> = Vec::new();

    for line in input.lines() {
        for (col_idx, value) in line.split_whitespace().enumerate() {
            if col_idx >= columns.len() {
                columns.push(Vec::new());
            }
            columns[col_idx].push(value);
        }
    }

    for column in columns {
        let operator: Operation = match column.last().unwrap() {
            &"+" => Operation::Add,
            &"*" => Operation::Multiply,
            _ => panic!("unknown type"),
        };
        let mut internal_result: u64 = match operator {
            Operation::Add => 0,
            Operation::Multiply => 1,
        };

        for &i in &column[..column.len() - 1] {
            let number = i.parse::<u64>().unwrap();
            match operator {
                Operation::Add => internal_result += number,
                Operation::Multiply => internal_result *= number,
            }
        }

        total += internal_result;
    }

    Some(total)
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
