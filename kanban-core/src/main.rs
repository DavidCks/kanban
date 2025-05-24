mod actions;
mod init;
mod schema;
mod ui;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    init::ensure_board_initialized()?; // Initialize if not present

    loop {
        match ui::prompt_main_menu()? {
            ui::Action::Move => actions::move_task::run()?,
            ui::Action::Add => actions::add_task::run()?,
            ui::Action::Edit => actions::edit_task::run()?,
            ui::Action::Delete => actions::delete_task::run()?,
            ui::Action::List => actions::list_board::run()?,
            ui::Action::Quit => break,
        }
    }

    Ok(())
}
