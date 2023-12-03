use regex::{Captures, Regex};

pub struct Set {
    pub blue: u32,
    pub red: u32,
    pub green: u32,
}

pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

fn parse_capture(c: Captures) -> u32 {
    c.get(1)
        .unwrap()
        .as_str()
        .parse()
        .expect("Failed to parse digit")
}

pub fn parse_game(line: &str) -> Game {
    let (metadata, sets) = line.split_once(':').unwrap();
    let id: u32 = metadata
        .split_once(' ')
        .unwrap()
        .1
        .parse()
        .expect("Failed to parse game id");

    let sets = sets.split(';').map(str::trim);
    let mut parsed_sets = vec![];
    let blue_pattern = Regex::new(r#"(\d+) blue"#).expect("Failed to build blue pattern");
    let red_pattern = Regex::new(r#"(\d+) red"#).expect("Failed to build red pattern");
    let green_pattern = Regex::new(r#"(\d+) green"#).expect("Failed to build green pattern");
    for set in sets {
        let blue = blue_pattern.captures(set).map(parse_capture).unwrap_or(0);
        let red = red_pattern.captures(set).map(parse_capture).unwrap_or(0);
        let green = green_pattern.captures(set).map(parse_capture).unwrap_or(0);
        parsed_sets.push(Set { blue, red, green });
    }

    Game {
        id,
        sets: parsed_sets,
    }
}

#[cfg(test)]
mod tests {
    use super::parse_game;

    #[test]
    fn test_parse_game_1() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse_game(line);

        assert_eq!(1, game.id);
        assert_eq!(3, game.sets.len());

        let sets = game.sets;
        let set1 = sets.get(0).unwrap();
        assert_eq!(3, set1.blue);
        assert_eq!(4, set1.red);
        assert_eq!(0, set1.green);

        let set2 = sets.get(1).unwrap();
        assert_eq!(6, set2.blue);
        assert_eq!(1, set2.red);
        assert_eq!(2, set2.green);

        let set3 = sets.get(2).unwrap();
        assert_eq!(0, set3.blue);
        assert_eq!(0, set3.red);
        assert_eq!(2, set3.green);
    }
}
