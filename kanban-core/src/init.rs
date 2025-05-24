use crate::schema::KanbanBoard;
use dialoguer::Confirm;
use std::fs;
use std::path::PathBuf;

pub fn ensure_board_initialized() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(".vscode/kanban.json");

    if !path.exists() {
        if Confirm::new()
            .with_prompt("No kanban board found. Create one?")
            .interact()?
        {
            fs::create_dir_all(".vscode")?;
            let board = KanbanBoard::default();
            fs::write(&path, serde_json::to_string_pretty(&board)?)?;
            println!("✅ Created .vscode/kanban.json");
        } else {
            println!("❌ Board not created. Exiting.");
            std::process::exit(0);
        }
    }

    Ok(())
}
