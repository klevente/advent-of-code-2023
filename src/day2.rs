use advent_of_code_2023::read_file_lines_as;
use std::str::FromStr;

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

#[derive(Debug)]
struct Game {
    id: u32,
    turns: Vec<Turn>,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.turns.iter().all(Turn::is_valid)
    }

    fn min_number_of_cubes(&self) -> (u32, u32, u32) {
        self.turns.iter().fold((0, 0, 0), |(r, g, b), next| {
            (r.max(next.reds), g.max(next.greens), b.max(next.blues))
        })
    }

    fn power_of_min_cubes(&self) -> u32 {
        let (r, g, b) = self.min_number_of_cubes();
        r * g * b
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(": ").ok_or(())?;
        let (_, id) = first.split_once(' ').ok_or(())?;

        let id = u32::from_str(id).map_err(|_| ())?;

        let turns = second
            .split(';')
            .map(Turn::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { id, turns })
    }
}

#[derive(Debug)]
struct Turn {
    reds: u32,
    greens: u32,
    blues: u32,
}

impl Turn {
    fn is_valid(&self) -> bool {
        self.reds <= RED && self.greens <= GREEN && self.blues <= BLUE
    }
}

impl FromStr for Turn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reds = 0;
        let mut greens = 0;
        let mut blues = 0;
        for split in s.split(',') {
            let (num, color) = split.trim().split_once(' ').ok_or(())?;

            let num = u32::from_str(num).map_err(|_| ())?;

            match color {
                "red" => reds = num,
                "green" => greens = num,
                "blue" => blues = num,
                _ => return Err(()),
            }
        }

        Ok(Self {
            reds,
            greens,
            blues,
        })
    }
}

fn calculate_sum_of_valid_game_ids(games: &[Game]) -> u32 {
    games
        .iter()
        .filter_map(|g| if g.is_valid() { Some(g.id) } else { None })
        .sum()
}

fn calculate_sum_of_minimum_cube_powers(games: &[Game]) -> u32 {
    games.iter().map(|g| g.power_of_min_cubes()).sum()
}

fn main() {
    let games = read_file_lines_as("input/day2.txt", |l| Game::from_str(l).unwrap());

    let sum_of_valid_game_ids = calculate_sum_of_valid_game_ids(&games);
    println!("The sum of valid game IDs is {sum_of_valid_game_ids}");

    let sum_of_minimum_cube_powers = calculate_sum_of_minimum_cube_powers(&games);
    println!(
        "The sum of the powers of the minimum required cubes is: {sum_of_minimum_cube_powers}"
    );
}
