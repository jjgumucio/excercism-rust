extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn is_yell(message: Vec<&str>) -> bool {
    // Numbers and special chars are qualifiying as uppercase...
    for c in message {
        if c != c.to_uppercase() {
            return false
        }
    }
    true
}

pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    if trimmed.is_empty() { return "Fine. Be that way!" }
    let as_graphemes = trimmed.graphemes(true).collect::<Vec<&str>>();
    let end_char = as_graphemes[as_graphemes.len() - 1];
    match is_yell(as_graphemes) {
        true => match end_char {
            "?" => return "Calm down, I know what I'm doing!",
            _ => return "Whoa, chill out!"
        }
        false => match end_char {
            "?" => return "Sure.",
            _ => return "Whatever."
        }
    }
}
