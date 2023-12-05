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

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let lines = file.lines().map(String::from).collect::<Vec<String>>();

    let first = first_part(&lines);
    let second = second_part(&lines);

    println!("first part:{}", first);
    println!("second part:{}", second);
}

fn first_part(lines: &Vec<String>) -> u32 {
    let mut values = vec![];
    for line in lines.to_owned() {
        let first = find_first(&line).unwrap();
        let last = find_first(&reverse_line!(line)).unwrap_or(first);

        values.push(first * 10 + last)
    }

    values.iter().sum()
}

fn second_part(lines: &Vec<String>) -> u32 {
    let reversed_table = reversed_mapping_table!();

    let mut values = vec![];

    let table = mapping_table!();

    for line in lines.to_owned() {
        let first = get_digit(&line, &table);

        let reversed_line = reverse_line!(line);

        let last = get_digit(&reversed_line, &reversed_table);

        values.push(first * 10 + last);
    }

    values.iter().sum()
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
    use crate::{first_part, get_digit, second_part};

    #[test]
    fn test_get_digit() {
        let input = vec!["8sadokptwo", "9asdioh24519023"];
        let mut result = vec![];
        for line in input {
            let digit = get_digit(line, &mapping_table!());
            result.push(digit)
        }

        assert_eq!(17 as u32, result.iter().sum())
    }

    #[test]
    fn test_reverse_get_digit() {
        let input = vec!["abcone", "blink1eighttwo"];

        let mut result = vec![];
        for line in input {
            let reverse_line = line.chars().rev().map(String::from).collect::<String>();
            let digit = get_digit(&reverse_line, &reversed_mapping_table!());
            result.push(digit)
        }

        assert_eq!(3 as u32, result.iter().sum())
    }

    #[test]
    fn test_first_part() {
        let input = vec!["avenged7fold", "18tolife", "3and7"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();

        let total = first_part(&input);

        assert_eq!(132, total)
    }

    #[test]
    fn test_second_part() {
        let input = vec![
            "avenged7fold",
            "18tolife",
            "3and7",
            "abcone",
            "blink1eigthtwo",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

        let total = second_part(&input);

        assert_eq!(155, total)
    }
}
