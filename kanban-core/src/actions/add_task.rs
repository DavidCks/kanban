use crate::schema::{KanbanBoard, Task};
use crate::ui::styled_title; // make sure styled_title is `pub` and accessible here
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

pub fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(".vscode/kanban.json");
    let contents = fs::read_to_string(&path)?;
    let mut board: KanbanBoard = serde_json::from_str(&contents)?;

    // Prompt for task text
    let text: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter task text")
        .interact_text()?;

    // Color column titles
    let col_titles: Vec<String> = board
        .cols
        .iter()
        .map(|col| styled_title(&col.title, &col.color).to_string())
        .collect();

    // Prompt for column selection
    let selected_col_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Add to which column?")
        .items(&col_titles)
        .interact()?;

    // Create and insert task
    let task = Task {
        text,
        id: Uuid::new_v4().to_string(),
    };
    board.cols[selected_col_index].tasks.push(task);

    // Save
    fs::write(path, serde_json::to_string_pretty(&board)?)?;
    println!(
        "✅ Task added to '{}'",
        board.cols[selected_col_index].title
    );

    Ok(())
}
