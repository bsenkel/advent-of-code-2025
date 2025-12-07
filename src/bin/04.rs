advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut paper_rolls_ok = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let cell = grid[row][col];
            if cell == '.' {
                continue;
            }

            let mut paper_rolls = 0;

            if row > 0 && col > 0 && grid[row - 1][col - 1] == '@' {
                paper_rolls += 1;
            }
            if row > 0 && grid[row - 1][col] == '@' {
                paper_rolls += 1;
            }
            if row > 0 && col + 1 < grid[row].len() && grid[row - 1][col + 1] == '@' {
                paper_rolls += 1;
            }
            if col > 0 && grid[row][col - 1] == '@' {
                paper_rolls += 1;
            }
            if col + 1 < grid[row].len() && grid[row][col + 1] == '@' {
                paper_rolls += 1;
            }
            if row + 1 < grid.len() && col > 0 && grid[row + 1][col - 1] == '@' {
                paper_rolls += 1;
            }
            if row + 1 < grid.len() && grid[row + 1][col] == '@' {
                paper_rolls += 1;
            }
            if row + 1 < grid.len() && col + 1 < grid[row].len() && grid[row + 1][col + 1] == '@' {
                paper_rolls += 1;
            }

            if paper_rolls < 4 {
                paper_rolls_ok += 1;
            }
        }
    }

    Some(paper_rolls_ok)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
