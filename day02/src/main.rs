use commons::{answer::Answer, load_input, solution::Solution};

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

const COMMA_DELIMITER: &str = ",";
const SEMICOLON_DELIMITER: &str = ";";
const COLON_DELIMITER: &str = ":";
const SPACE_DELIMITER: &str = " ";

macro_rules! split {
    ($line:expr, $delimiter:expr) => {
        $line
            .split($delimiter)
            .map(|p| p.trim())
            .collect::<Vec<&str>>()
    };
}

struct Day02;

impl Solution for Day02 {
    fn first_part(&self, input: &str) -> Answer {
        let lines = input.lines();
        let mut sum = 0;

        'main: for line in lines {
            let (game, sets) = split_game(line);
            let combinations = sets
                .iter()
                .flat_map(|set| split!(set, SEMICOLON_DELIMITER))
                .collect::<Vec<&str>>();

            for combination in &combinations {
                let cubes_per_set = split!(combination, COMMA_DELIMITER);

                for cubes in &cubes_per_set {
                    let parts = split!(cubes, SPACE_DELIMITER);
                    let color = parts[1];
                    let number = parts[0].parse::<u32>().unwrap();

                    let is_valid = match color {
                        "red" => number <= RED_MAX,
                        "green" => number <= GREEN_MAX,
                        "blue" => number <= BLUE_MAX,
                        _ => panic!("unknown color"),
                    };

                    if !is_valid {
                        continue 'main;
                    }
                }
            }

            sum += split!(game, SPACE_DELIMITER)[1].parse::<u32>().unwrap();
        }

        Answer::Number(sum)
    }

    fn second_part(&self, input: &str) -> Answer {
        let lines = input.lines();
        let mut sum = 0;

        for line in lines {
            let mut minimun_red = 0;
            let mut minimun_green = 0;
            let mut minimun_blue = 0;

            let (_, sets) = split_game(line);
            let combinations = sets
                .iter()
                .flat_map(|set| split!(set, SEMICOLON_DELIMITER))
                .collect::<Vec<&str>>();

            for combination in &combinations {
                let cubes_per_set = split!(combination, COMMA_DELIMITER);

                for cubes in &cubes_per_set {
                    let parts = split!(cubes, SPACE_DELIMITER);
                    let color = parts[1];
                    let number = parts[0].parse::<u32>().unwrap();

                    match color {
                        "red" => minimun_red = minimun_red.max(number),
                        "green" => minimun_green = minimun_green.max(number),
                        "blue" => minimun_blue = minimun_blue.max(number),
                        _ => panic!("unknown color"),
                    };
                }
            }
            let power = minimun_red * minimun_green * minimun_blue;
            sum += power;
        }

        Answer::Number(sum)
    }
}

fn split_game(line: &str) -> (&str, Vec<&str>) {
    let parts = split!(line.trim(), COLON_DELIMITER);
    (parts[0], split!(parts[1], SEMICOLON_DELIMITER))
}

fn main() {
    let file = load_input!();

    let day2 = Day02;

    let first_part = day2.first_part(&file);
    let second_part = day2.second_part(&file);

    println!("{:?}", first_part);
    println!("{:?}", second_part);
}

#[cfg(test)]
mod test {
    use crate::Day02;
    use commons::solution::Solution;

    #[test]
    fn test_first_part() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let day02 = Day02;

        let answer = day02.first_part(&input.join("\n"));

        assert_eq!(answer, 8.into())
    }

    #[test]
    fn test_second_part() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let day02 = Day02;

        let answer = day02.second_part(&input.join("\n"));

        assert_eq!(answer, 2286.into())
    }
}
