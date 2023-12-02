use crate::Solution;

#[derive(Default)]
pub struct Day2 {}

impl Solution for Day2 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day2");
        determine_possible_rounds(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day2");
        determine_minimum_cubes(input)
    }
}

fn determine_possible_rounds(input: &str) -> usize {
    let mut possible_game_ids_sum = 0;
    for line in input.lines() {
        let (game, rest) = line.split_once(": ").unwrap();
        let (_, game_id) = game.split_once(' ').unwrap();
        let game_id: usize = game_id.parse().unwrap();
        let rounds_str = rest.split("; ");
        let mut rounds = rounds_str.map(cubes_from_round_str);
        if rounds.all(is_round_possible) {
            possible_game_ids_sum += game_id;
        }
    }
    possible_game_ids_sum
}

type RGB = (usize, usize, usize);

fn cubes_from_round_str(round: &str) -> RGB {
    let mut cubes: RGB = (0, 0, 0);
    for cube_str in round.split(", ") {
        let (count, color) = cube_str.split_once(' ').unwrap();
        let count: usize = count.parse().unwrap();
        match color {
            "red" => cubes.0 = count,
            "green" => cubes.1 = count,
            "blue" => cubes.2 = count,
            _ => panic!("invalid color: {color}"),
        };
    }
    cubes
}

fn is_round_possible((r, g, b): RGB) -> bool {
    12 >= r && 13 >= g && 14 >= b
}

fn determine_minimum_cubes(input: &str) -> usize {
    let mut power_sum = 0;
    for line in input.lines() {
        let (_, rest) = line.split_once(": ").unwrap();
        let rounds_str = rest.split("; ");
        let mut minimum_set = (0, 0, 0);
        let rounds = rounds_str.map(cubes_from_round_str);
        for cubes in rounds {
            minimum_set.0 = minimum_set.0.max(cubes.0);
            minimum_set.1 = minimum_set.1.max(cubes.1);
            minimum_set.2 = minimum_set.2.max(cubes.2);
        }
        power_sum += minimum_set.0 * minimum_set.1 * minimum_set.2
    }
    power_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(determine_possible_rounds(input), 8);
    }

    #[test]
    fn part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(determine_minimum_cubes(input), 2286);
    }
}
