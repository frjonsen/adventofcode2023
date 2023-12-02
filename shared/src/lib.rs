pub fn read_input(day: u8) -> Vec<String> {
    let pwd = std::env::current_dir().expect("Failed to get current directory");
    let full_path = pwd.join("inputs").join(format!("day{day}.txt"));
    let contents = std::fs::read_to_string(full_path)
        .unwrap_or_else(|_| panic!("Failed to read input for day {day}"));
    contents
        .split_whitespace()
        .map(str::trim)
        .map(str::to_owned)
        .collect()
}
