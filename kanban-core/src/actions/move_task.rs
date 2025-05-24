use crate::schema::KanbanBoard;
use crate::ui::styled_title;
use dialoguer::{theme::ColorfulTheme, Select};
use std::fs;
use std::path::PathBuf;

pub fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(".vscode/kanban.json");
    let contents = fs::read_to_string(&path)?;
    let mut board: KanbanBoard = serde_json::from_str(&contents)?;

    // Step 1: Prompt for column (or all)
    let mut col_options: Vec<String> = board
        .cols
        .iter()
        .map(|c| format!("{} ({})", styled_title(&c.title, &c.color), c.tasks.len()))
        .collect();
    col_options.insert(0, "All Columns".to_string());

    let selected_col_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Filter tasks by column")
        .items(&col_options)
        .default(0)
        .interact()?;

    // Step 2: Build task list for selected column(s)
    let mut task_refs = vec![];

    for (col_index, col) in board.cols.iter().enumerate() {
        if selected_col_index == 0 || selected_col_index - 1 == col_index {
            for task in &col.tasks {
                task_refs.push((task.clone(), col_index));
            }
        }
    }

    if task_refs.is_empty() {
        println!("No tasks found in selected column(s).");
        return Ok(());
    }

    let task_labels: Vec<String> = task_refs
        .iter()
        .map(|(task, _col_index)| format!("{}", task.text.replace('\n', " ")))
        .collect();

    // Step 3: Select task
    let selected_task_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a task to move")
        .items(&task_labels)
        .interact()?;

    let (task_to_move, source_col_index) = task_refs[selected_task_index].clone();

    // Step 4: Choose destination column
    let col_titles: Vec<String> = board
        .cols
        .iter()
        .map(|col| styled_title(&col.title, &col.color).to_string())
        .collect();

    let dest_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Move to which column?")
        .items(&col_titles)
        .default(source_col_index)
        .interact()?;

    if dest_index == source_col_index {
        println!(
            "Task already in '{}'. No move needed.",
            board.cols[dest_index].title
        );
        return Ok(());
    }

    // Step 5: Move task
    let task_id = &task_to_move.id;
    board.cols[source_col_index]
        .tasks
        .retain(|t| &t.id != task_id);

    board.cols[dest_index].tasks.push(task_to_move);

    fs::write(path, serde_json::to_string_pretty(&board)?)?;
    println!("✅ Moved task to '{}'", board.cols[dest_index].title);

    Ok(())
}
