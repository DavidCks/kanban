use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Task {
    pub text: String,
    pub id: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Column {
    pub title: String,
    pub tasks: Vec<Task>,
    pub id: String,
    pub color: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KanbanBoard {
    pub title: String,
    pub cols: Vec<Column>,
    pub autosave: bool,
    pub save_to_file: bool,
    pub timestamp: i64,
}

impl Default for KanbanBoard {
    fn default() -> Self {
        Self {
            title: "Kanban".to_string(),
            cols: vec![
                Column {
                    title: "Bugs".into(),
                    tasks: vec![],
                    id: generate_id(),
                    color: "#eb144c".into(),
                },
                Column {
                    title: "To-Do".into(),
                    tasks: vec![],
                    id: generate_id(),
                    color: "#fcb900".into(),
                },
                Column {
                    title: "Doing".into(),
                    tasks: vec![],
                    id: generate_id(),
                    color: "#0693e3".into(),
                },
                Column {
                    title: "Done".into(),
                    tasks: vec![],
                    id: generate_id(),
                    color: "#00d084".into(),
                },
            ],
            autosave: true,
            save_to_file: true,
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }
}

fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
