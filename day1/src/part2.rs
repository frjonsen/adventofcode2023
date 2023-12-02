use shared::read_input;

fn main() {
    let input = read_input(1);
    let result: u64 = run(input.iter());
    println!("{result}");
}

const VALID_MATCHES: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3",
    "4", "5", "6", "7", "8", "9",
];

fn translate_match(num: &str) -> u64 {
    match num {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => panic!("Invalid value"),
    }
}

fn find_first_match(line: &str) -> u64 {
    for i in 0..line.len() {
        for valid_match in VALID_MATCHES {
            if line[i..].starts_with(valid_match) {
                return translate_match(valid_match);
            }
        }
    }

    panic!("Failed to find first digit");
}

fn find_last_match(line: &str) -> u64 {
    for i in (0..line.len()).rev() {
        for valid_match in VALID_MATCHES {
            if line[i..].starts_with(valid_match) {
                return translate_match(valid_match);
            }
        }
    }

    panic!("Failed to find last digit")
}

fn run<I>(input: I) -> u64
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut sum = 0;
    for line in input {
        let line = line.as_ref();
        let first = find_first_match(line);
        let last = find_last_match(line);

        sum += first * 10 + last;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::{find_first_match, find_last_match, run};

    #[test]
    fn test_find_first() {
        let line = "wo1nine";
        let res = find_first_match(line);
        assert_eq!(1, res);
    }

    #[test]
    fn test_find_last_not_at_end() {
        let line = "abcone2threexyz";
        let res = find_last_match(line);
        assert_eq!(3, res);
    }

    #[test]
    fn test_find_last_at_end() {
        let line = "abcone2three";
        let res = find_last_match(line);
        assert_eq!(3, res);
    }

    #[test]
    fn test_given() {
        let lines = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let result = run(lines);
        assert_eq!(281, result);
    }
}
