use colored::{ColoredString, Colorize};
use engine::strategy::PlayerAction;

pub fn action_to_string(action: &PlayerAction) -> String {
    String::from(match action {
        PlayerAction::Hit => "H",
        PlayerAction::Stand => "S",
        PlayerAction::DoubleOrHit => "DH",
        PlayerAction::DoubleOrStand => "DS",
        PlayerAction::Split => "P",
        PlayerAction::Surrender => "R",
    })
}

pub fn action_to_long_string(action: &PlayerAction) -> String {
    String::from(match action {
        PlayerAction::Hit => "Hit",
        PlayerAction::Stand => "Stand",
        PlayerAction::DoubleOrHit => "Double or Hit",
        PlayerAction::DoubleOrStand => "Double or Stand",
        PlayerAction::Split => "Split",
        PlayerAction::Surrender => "Surrender",
    })
}

fn colored_string(s: String, action: &PlayerAction) -> ColoredString {
    match action {
        PlayerAction::Hit => s.red(),
        PlayerAction::Stand => s.yellow(),
        PlayerAction::DoubleOrHit => s.cyan(),
        PlayerAction::DoubleOrStand => s.cyan(),
        PlayerAction::Split => s.green(),
        PlayerAction::Surrender => s.white(),
    }
    .bold()
}

pub fn action_to_colored_string(action: &PlayerAction) -> ColoredString {
    let s = action_to_string(action);
    colored_string(s, action)
}

pub fn action_to_long_colored_string(action: &PlayerAction) -> ColoredString {
    let s = action_to_long_string(action);
    colored_string(s, action)
}
