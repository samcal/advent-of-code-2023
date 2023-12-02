use std::cmp;
use std::fs;

use advent_of_code_2023::{mconcat, Monoid, Semigroup};

fn main() {
    let input = fs::read_to_string("inputs/day02").unwrap();
    let games: Vec<Game> = input
        .lines()
        .map(|line| Game::parse(line).unwrap())
        .collect();

    let hypothetical_set = CubeSet::parse("12 red, 13 green, 14 blue").unwrap();
    let solution1: u32 = games
        .iter()
        .filter(|game| game.revelations_subset_of(&hypothetical_set))
        .map(|game| game.id)
        .sum();
    println!("solution for part one: {}", solution1);

    let solution2: u32 = games
        .iter()
        .map(|game| game.smallest_covering_set().power())
        .sum();
    println!("solution for part two: {}", solution2);
}

struct Game {
    id: u32,
    revealed_cube_sets: Vec<CubeSet>,
}

impl Game {
    fn parse(raw_game: &str) -> Option<Game> {
        let (game_label, cube_sets) = raw_game.split_once(": ")?;
        let (_, raw_id) = game_label.split_once(' ')?;
        let id = raw_id.parse().ok()?;
        let revealed_cube_sets = cube_sets
            .split("; ")
            .map(|raw_cube_set| CubeSet::parse(raw_cube_set).unwrap())
            .collect();
        let game = Game {
            id,
            revealed_cube_sets,
        };
        Some(game)
    }

    fn revelations_subset_of(&self, other_set: &CubeSet) -> bool {
        self.revealed_cube_sets
            .iter()
            .all(|cube_set| cube_set.is_subset_of(other_set))
    }

    fn smallest_covering_set(&self) -> CubeSet {
        mconcat(self.revealed_cube_sets.clone().into_iter())
    }
}

#[derive(Copy, Clone)]
struct CubeSet {
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}

impl Semigroup for CubeSet {
    fn mappend(&self, b: &Self) -> Self {
        CubeSet {
            red_count: cmp::max(self.red_count, b.red_count),
            green_count: cmp::max(self.green_count, b.green_count),
            blue_count: cmp::max(self.blue_count, b.blue_count),
        }
    }
}

impl Monoid for CubeSet {
    fn mempty() -> Self {
        CubeSet {
            red_count: 0,
            green_count: 0,
            blue_count: 0,
        }
    }
}

impl CubeSet {
    fn parse(raw_cubes: &str) -> Option<CubeSet> {
        Some(mconcat(raw_cubes.split(", ").map(|raw_color_count| {
            let (raw_count, raw_color) = raw_color_count.split_once(' ').unwrap();
            let count: u32 = raw_count.parse().ok().unwrap();
            match raw_color {
                "red" => CubeSet {
                    red_count: count,
                    green_count: 0,
                    blue_count: 0,
                },
                "green" => CubeSet {
                    red_count: 0,
                    green_count: count,
                    blue_count: 0,
                },
                "blue" => CubeSet {
                    red_count: 0,
                    green_count: 0,
                    blue_count: count,
                },
                _ => CubeSet::mempty(),
            }
        })))
    }

    fn is_subset_of(&self, other_set: &CubeSet) -> bool {
        self.red_count <= other_set.red_count
            && self.green_count <= other_set.green_count
            && self.blue_count <= other_set.blue_count
    }

    fn power(&self) -> u32 {
        self.red_count * self.green_count * self.blue_count
    }
}

#[test]
fn test_game_parse() {
    let game =
        Game::parse("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red")
            .unwrap();
    assert_eq!(game.id, 4);
    assert_eq!(game.revealed_cube_sets[0].blue_count, 6);
    assert_eq!(game.revealed_cube_sets[1].green_count, 3);
    assert_eq!(game.revealed_cube_sets[2].red_count, 14);
}

#[test]
fn test_cube_set_parse() {
    let cube_set = CubeSet::parse("3 blue, 4 red").unwrap();
    assert_eq!(cube_set.blue_count, 3);
    assert_eq!(cube_set.red_count, 4);
    assert_eq!(cube_set.green_count, 0);
}
