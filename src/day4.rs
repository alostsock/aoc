use crate::Solution;

#[derive(Default)]
pub struct Day4 {}

impl Solution for Day4 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day4");
        let grid = grid_from_input(input);
        num_rolls_accessible(&grid)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day4");
        num_rolls_accessible_with_removal(input)
    }
}

fn grid_from_input(input: &str) -> Vec<Vec<bool>> {
    input.lines().into_iter().map(|line| {
        line.chars().map(|c| c == '@').collect()
    }).collect()
}

fn adj_coords(i: usize, j: usize, i_max: usize, j_max: usize) -> Vec<(usize, usize)> {
    let mut coords = vec![];
    if j > 0 {
        coords.push((i, j - 1));
    }
    if j < j_max {
        coords.push((i, j + 1));
    }

    if i > 0 {
        coords.push((i - 1, j));
        if j > 0 {
            coords.push((i - 1, j - 1));
        }
        if j < j_max {
            coords.push((i - 1, j + 1));
        }
    }
    if i < i_max {
        coords.push((i + 1, j));
        if j > 0 {
            coords.push((i + 1, j - 1));
        }
        if j < j_max {
            coords.push((i + 1, j + 1));
        }
    }
    coords
}


fn num_rolls_accessible(grid: &Vec<Vec<bool>>) -> usize {
    let mut num_rolls = 0;

    let j_max = grid.len() - 1;
    let i_max = grid[0].len() - 1;
    for j in 0..=j_max {
        for i in 0..=i_max {
            if !grid[j][i] {
                continue;
            }

            let adjacent_rolls = adj_coords(i, j, i_max, j_max).iter().filter(|coords| {
                grid[coords.1][coords.0]
            }).count();

            if adjacent_rolls < 4 {
                num_rolls += 1;
            }
        }
    }

    num_rolls
}

fn remove_rolls(grid: &mut Vec<Vec<bool>>) -> usize {
    let mut num_removed = 0;

    let j_max = grid.len() - 1;
    let i_max = grid[0].len() - 1;
    for j in 0..=j_max {
        for i in 0..=i_max {
            if !grid[j][i] {
                continue;
            }

            let adjacent_rolls = adj_coords(i, j, i_max, j_max).iter().filter(|coords| {
                grid[coords.1][coords.0]
            }).count();

            if adjacent_rolls < 4 {
                grid[j][i] = false;
                num_removed += 1;
            }
        }
    }

    num_removed
}

fn num_rolls_accessible_with_removal(input: &str) -> usize {
    let mut grid = grid_from_input(input);

    let initial_roll_count = grid.iter().flatten().filter(|r| **r).count();

    loop {
        let num_removed = remove_rolls(&mut grid);
        if num_removed == 0 {
            break;
        }
    }

    let final_roll_count = grid.iter().flatten().filter(|r| **r).count();

    initial_roll_count - final_roll_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let grid = grid_from_input(input);

        assert_eq!(num_rolls_accessible(&grid), 13);
    }

    #[test]
    fn part_2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(num_rolls_accessible_with_removal(&input), 43);
    }
}
