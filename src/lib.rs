pub fn twofer(name: &str) -> String {
    if name.is_empty() {
        return String::from("One for you, one for me.");
    }
    format!("One for {}, one for me.", name)
}
