use shared::read_input;

fn run<I>(input: I) -> u64
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut sum = 0;
    for line in input.into_iter() {
        let first = line
            .as_ref()
            .chars()
            .find(|c| c.is_ascii_digit())
            .expect("Failed to find first digit");
        let last = line
            .as_ref()
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .expect("Failed to find last digit");
        let f = first as u64 - 48;
        let l = last as u64 - 48;
        sum += f * 10 + l;
    }

    sum
}

fn main() {
    let input = read_input(1);
    let result: u64 = run(input.iter());
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn test_given() {
        let lines = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let result = run(lines);
        assert_eq!(142, result);
    }
}
