use std::cmp;
use std::fs;

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
        let mut smallest_cube_set = CubeSet::empty();
        for cube_set in self.revealed_cube_sets.iter() {
            smallest_cube_set.red_count = cmp::max(smallest_cube_set.red_count, cube_set.red_count);
            smallest_cube_set.green_count =
                cmp::max(smallest_cube_set.green_count, cube_set.green_count);
            smallest_cube_set.blue_count =
                cmp::max(smallest_cube_set.blue_count, cube_set.blue_count);
        }
        smallest_cube_set
    }
}

struct CubeSet {
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}

impl CubeSet {
    fn empty() -> CubeSet {
        CubeSet {
            red_count: 0,
            green_count: 0,
            blue_count: 0,
        }
    }

    fn parse(raw_cubes: &str) -> Option<CubeSet> {
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;
        for raw_color_count in raw_cubes.split(", ") {
            let (raw_count, raw_color) = raw_color_count.split_once(' ')?;
            let count = raw_count.parse().ok()?;
            match raw_color {
                "red" => red_count = count,
                "green" => green_count = count,
                "blue" => blue_count = count,
                _ => return None,
            }
        }
        let cube_set = CubeSet {
            red_count,
            green_count,
            blue_count,
        };
        Some(cube_set)
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
