use day_share::parse_game;
use shared::read_input;

mod day_share;

fn run<I>(lines: I) -> u32
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let games = lines.into_iter().map(|g| parse_game(g.as_ref()));
    let mut sum = 0;
    for game in games.into_iter() {
        let mut blue_max = u32::MIN;
        let mut red_max = u32::MIN;
        let mut green_max = u32::MIN;

        for set in game.sets {
            blue_max = blue_max.max(set.blue);
            green_max = green_max.max(set.green);
            red_max = red_max.max(set.red);
        }
        sum += blue_max * green_max * red_max;
    }

    sum
}

fn main() {
    let input = read_input(2);
    let result = run(input);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn test_example() {
        let input = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let result = run(input);
        assert_eq!(2286, result);
    }
}
