use crate::schema::{Column, KanbanBoard};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use std::fs;
use std::path::PathBuf;

const COL_WIDTH: usize = 30;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(".vscode/kanban.json");
    let contents = fs::read_to_string(&path)?;
    let board: KanbanBoard = serde_json::from_str(&contents)?;

    let mut options: Vec<String> = board
        .cols
        .iter()
        .map(|col| format!("{} ({})", col.title, col.tasks.len()))
        .collect();
    options.insert(0, "All Columns".to_string());

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a column to view")
        .items(&options)
        .default(0)
        .interact()?;

    println!("\n📋  {}\n", board.title.bold());

    if selection == 0 {
        print_board_table(&board.cols);
    } else {
        let col = &board.cols[selection - 1];
        print_single_column(col);
    }

    Ok(())
}

fn print_single_column(col: &Column) {
    println!(
        "{}\n{}",
        styled_title(&col.title, &col.color),
        "─".repeat(COL_WIDTH)
    );

    for task in &col.tasks {
        println!("• {}", task.text.replace('\n', " "));
    }

    println!();
}

fn print_board_table(cols: &[Column]) {
    for col in cols {
        print!(
            "{:<width$}",
            styled_title(&col.title, &col.color),
            width = COL_WIDTH
        );
    }
    println!();

    for _ in cols {
        print!("{:<width$}", "─".repeat(COL_WIDTH - 2), width = COL_WIDTH);
    }
    println!();

    let max_rows = cols.iter().map(|c| c.tasks.len()).max().unwrap_or(0);

    for row_index in 0..max_rows {
        let wrapped_lines_per_col: Vec<Vec<String>> = cols
            .iter()
            .map(|col| {
                if let Some(task) = col.tasks.get(row_index) {
                    wrap_text(&format!("• {}", task.text), COL_WIDTH)
                } else {
                    vec!["".to_string()]
                }
            })
            .collect();

        let max_lines = wrapped_lines_per_col
            .iter()
            .map(|lines| lines.len())
            .max()
            .unwrap_or(1);

        for line_index in 0..max_lines {
            for col_lines in &wrapped_lines_per_col {
                let line = col_lines.get(line_index).cloned().unwrap_or_default();
                print!("{:<width$}", line, width = COL_WIDTH);
            }
            println!();
        }
    }
}

fn styled_title(title: &str, color: &str) -> ColoredString {
    match color {
        "#eb144c" => title.red().bold(),
        "#fcb900" => title.yellow().bold(),
        "#0693e3" => title.blue().bold(),
        "#00d084" => title.green().bold(),
        _ => title.normal(),
    }
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = vec![];
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.len() + word.len() + 1 > width {
            lines.push(current.clone());
            current.clear();
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}
