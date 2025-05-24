use crate::schema::KanbanBoard;
use crate::ui::styled_title;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::fs;
use std::path::PathBuf;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(".vscode/kanban.json");
    let contents = fs::read_to_string(&path)?;
    let mut board: KanbanBoard = serde_json::from_str(&contents)?;

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

    let mut task_refs: Vec<(usize, usize)> = vec![];
    let mut labels: Vec<String> = vec![];

    for (col_index, col) in board.cols.iter().enumerate() {
        if target_col_id.is_none() || Some(&col.id) == target_col_id.as_ref() {
            for (task_index, task) in col.tasks.iter().enumerate() {
                task_refs.push((task_index, col_index));
                let colored_col = styled_title(&col.title, &col.color).to_string();
                labels.push(format!("[{}] {}", colored_col, task.text));
            }
        }
    }

    if labels.is_empty() {
        println!("No tasks to delete.");
        return Ok(());
    }

    let selected = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a task to delete")
        .items(&labels)
        .interact()?;

    let (task_index, col_index) = task_refs[selected];
    let task = &board.cols[col_index].tasks[task_index];

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Are you sure you want to delete '{}'? ", task.text))
        .default(false)
        .interact()?;

    if !confirm {
        println!("❌ Task not deleted.");
        return Ok(());
    }

    board.cols[col_index].tasks.remove(task_index);
    fs::write(path, serde_json::to_string_pretty(&board)?)?;
    println!("✅ Task deleted.");
    Ok(())
}
