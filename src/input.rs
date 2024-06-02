use std::io;

pub fn confirm(text: String) -> bool {
    let mut input = String::new();
    println!("{} (y):", text);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes" | "")
}

pub fn input(text: &str, default_value: Option<String>) -> Option<String> {
    let mut input = String::new();

    let default = match default_value {
        Some(ref default) => format!(" ({})", default),
        _ => "".to_owned(),
    };

    println!("{}{}:", text, default);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim() {
        "" => default_value.clone(),
        input => Some(input.to_owned()),
    }
}
