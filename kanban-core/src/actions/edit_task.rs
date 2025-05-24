use crate::schema::{KanbanBoard, Task};
use crate::ui::styled_title;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::fs;
use std::path::PathBuf;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(".vscode/kanban.json");
    let contents = fs::read_to_string(&path)?;
    let mut board: KanbanBoard = serde_json::from_str(&contents)?;

    // Select column
    let mut col_options: Vec<String> = board
        .cols
        .iter()
        .map(|c| format!("{} ({})", styled_title(&c.title, &c.color), c.tasks.len()))
        .collect();
    col_options.insert(0, "All Columns".to_string());

    let col_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a column to filter tasks")
        .items(&col_options)
        .default(0)
        .interact()?;

    let target_col_id = if col_choice == 0 {
        None
    } else {
        Some(board.cols[col_choice - 1].id.clone())
    };

    let mut task_refs: Vec<(&mut Task, String)> = vec![];

    for col in &mut board.cols {
        if target_col_id.is_none() || Some(&col.id) == target_col_id.as_ref() {
            let styled = styled_title(&col.title, &col.color).to_string();
            for task in &mut col.tasks {
                task_refs.push((task, styled.clone()));
            }
        }
    }

    if task_refs.is_empty() {
        println!("No tasks to edit.");
        return Ok(());
    }

    let task_labels: Vec<String> = task_refs
        .iter()
        .map(|(task, styled)| format!("[{}] {}", styled, task.text))
        .collect();

    let selected = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a task to edit")
        .items(&task_labels)
        .interact()?;

    let (task, _) = &mut task_refs[selected];

    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Edit task text (leave blank to cancel)")
        .with_initial_text(task.text.clone())
        .interact_text()?;

    task.text = input;

    fs::write(path, serde_json::to_string_pretty(&board)?)?;
    println!("✅ Task updated.");
    Ok(())
}
