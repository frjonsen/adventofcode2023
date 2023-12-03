use day_share::{parse_game, Set};
use shared::read_input;

mod day_share;

fn run<I>(lines: I) -> u32
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let available_set = Set {
        blue: 14,
        green: 13,
        red: 12,
    };

    let mut legal_games = 0;

    let games = lines.into_iter().map(|g| parse_game(g.as_ref()));

    for game in games {
        let sets_legal = game.sets.iter().all(|s| {
            s.green <= available_set.green
                && s.blue <= available_set.blue
                && s.red <= available_set.red
        });
        if sets_legal {
            legal_games += game.id;
        }
    }

    legal_games
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
    fn test_sample() {
        let input = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let result = run(input);
        assert_eq!(8, result);
    }
}
