use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Debug)]
pub enum Action {
    Move,
    Add,
    Edit,
    Delete,
    List,
    Quit,
}

pub fn styled_title(title: &str, color: &str) -> ColoredString {
    match color {
        "#eb144c" => title.red().bold(),
        "#fcb900" => title.yellow().bold(),
        "#0693e3" => title.blue().bold(),
        "#00d084" => title.green().bold(),
        _ => title.normal(),
    }
}

pub fn prompt_main_menu() -> std::io::Result<Action> {
    let actions = vec!["List", "Move", "Add", "Edit", "Delete", "Quit"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an action")
        .items(&actions)
        .default(0)
        .interact()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(match actions[selection] {
        "List" => Action::List,
        "Move" => Action::Move,
        "Add" => Action::Add,
        "Edit" => Action::Edit,
        "Delete" => Action::Delete,
        _ => Action::Quit,
    })
}
