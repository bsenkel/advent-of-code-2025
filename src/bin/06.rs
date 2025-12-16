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
    let mut total: u64 = 0;
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut current_group: Vec<String> = Vec::new();
    let mut transposed: Vec<String> = Vec::new();

    let (operands, operators) = input.trim_end().rsplit_once("\n")?;
    let char_matrix: Vec<Vec<char>> = operands
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let num_cols = char_matrix.iter().map(|row| row.len()).max().unwrap_or(0);
    for col_idx in 0..num_cols {
        let column: String = char_matrix
            .iter()
            .filter_map(|row| row.get(col_idx).copied())
            .collect();
        transposed.push(column);
    }

    let operator_list: Vec<&str> = operators.split_whitespace().collect();

    for trans_line in transposed {
        if trans_line.trim().is_empty() {
            if !current_group.is_empty() {
                groups.push(current_group);
                current_group = Vec::new();
            }
        } else {
            current_group.push(trans_line);
        }
    }

    if !current_group.is_empty() {
        groups.push(current_group);
    }

    for (group, op_str) in groups.iter().zip(operator_list) {
        let operator = match op_str {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => continue,
        };

        let numbers: Vec<u64> = group.iter().filter_map(|s| s.trim().parse().ok()).collect();

        let result: u64 = match operator {
            Operation::Add => numbers.iter().sum(),
            Operation::Multiply => numbers.iter().product(),
        };

        total += result;
    }

    Some(total)
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
        assert_eq!(result, Some(3263827));
    }
}
