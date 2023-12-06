use commons::{answer::Answer, load_input, solution::Solution};

const MAPPING_TABLE: &[(u32, &str)] = &[
    (0, "zero"),
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
    (0, "0"),
    (1, "1"),
    (2, "2"),
    (3, "3"),
    (4, "4"),
    (5, "5"),
    (6, "6"),
    (7, "7"),
    (8, "8"),
    (9, "9"),
];

macro_rules! mapping_table {
    () => {{
        use $crate::MAPPING_TABLE;

        MAPPING_TABLE
            .into_iter()
            .map(|(value, pattern)| (*value, pattern.to_string()))
            .collect()
    }};
}

macro_rules! reversed_mapping_table {
    () => {{
        use $crate::MAPPING_TABLE;
        MAPPING_TABLE
            .iter()
            .map(|(value, pattern)| (*value, pattern.chars().rev().collect::<String>()))
            .collect()
    }};
}

macro_rules! reverse_line {
    ($line:expr) => {
        $line.chars().rev().collect::<String>()
    };
}

struct Day01;

impl Solution for Day01 {
    fn first_part(&self, input: &str) -> Answer {
        let lines = input.lines();
        let mut values = vec![];
        for line in lines {
            let first = find_first(line).unwrap();
            let last = find_first(&reverse_line!(line)).unwrap_or(first);

            values.push(first * 10 + last)
        }

        let total: u32 = values.iter().sum();

        Answer::Number(total)
    }

    fn second_part(&self, input: &str) -> Answer {
        let lines = input.lines();
        let reversed_table = reversed_mapping_table!();

        let mut values = vec![];

        let table = mapping_table!();

        for line in lines {
            let first = get_digit(line, &table);

            let reversed_line = reverse_line!(line);

            let last = get_digit(&reversed_line, &reversed_table);

            values.push(first * 10 + last);
        }

        let total: u32 = values.iter().sum();

        Answer::Number(total)
    }
}

fn main() {
    let file = load_input!();

    let day01 = Day01;
    let first_part = day01.first_part(&file);
    let second_part = day01.second_part(&file);

    println!("first part:{:?}", first_part);
    println!("second part:{:?}", second_part);
}

fn find_first(line: &str) -> Option<u32> {
    line.chars().find_map(|c| c.to_digit(10))
}

fn get_digit(mut line: &str, values: &Vec<(u32, String)>) -> u32 {
    loop {
        for (value, pattern) in values {
            if line.starts_with(pattern) {
                return *value;
            }
        }
        line = &line[1..];
    }
}

#[cfg(test)]
mod test {
    use crate::Day01;
    use commons::solution::Solution;

    #[test]
    fn test_first_part() {
        let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();
        let day01 = Day01;

        let total = day01.first_part(&input.join("\n"));

        assert_eq!(total, 142.into())
    }

    #[test]
    fn test_second_part() {
        let input = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();
        let day01 = Day01;
        let total = day01.second_part(&input.join("\n"));

        assert_eq!(total, 281.into());
    }
}
