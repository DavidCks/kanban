# Kanban CLI :clipboard:

A terminal-based Kanban board manager with colorful UI, built in Rust.

## Features :sparkles:

- :rainbow: **Color-coded columns** for better visual organization
- :keyboard: **Interactive menus** with `dialoguer` for smooth navigation
- :floppy_disk: **Automatic saving** to `.vscode/kanban.json`
- :arrows_counterclockwise: **Full CRUD operations**:
  - Add, edit, move, and delete tasks
  - View tasks by column or all at once
- :package: **Easy installation** with a single script
- compatible with (vscode kanban)[https://marketplace.visualstudio.com/items?itemName=lbauskar.kanban&ssr=false#overview]

## Installation :computer:

### Quick Install (Linux/macOS)
```bash
curl -sSL https://raw.githubusercontent.com/DavidCks/kanban/main/install.sh | bash
```

## Manual Build

1. Ensure you have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone and build:

```bash
git clone https://github.com/DavidCks/kanban.git
cd kanban
cargo install --path .
```

## Usage :rocket:

kanban is interactive, so there are no commands you need to remember. Simply run:

```bash
kanban
```

## Preview

```bash
? Choose an action ›
❯ List
  Move
  Add
  Edit
  Delete
  Quit
```

```bash
✔ Choose an action · Add
✔ Enter task text · Hello
? Add to which column? ›
❯ Bugs
  To-Do
  Doing
  Done
```

## Data Structure :file_folder:

Tasks are saved in `.vscode/kanban.json`:

```json
{
  "title": "Kanban",
  "cols": [
    {
      "title": "Bugs",
      "color": "#eb144c",
      "tasks": [
        {"text": "Fix login bug", "id": "uuid..."}
      ]
    }
  ]
}
```

## Columns

- :red_circle: Bugs (#eb144c)#
- :yellow_circle: To-Do (#fcb900)
- :blue_circle: Doing (#0693e3)
- :green_circle: Done (#00d084)

## Dependencies :gear:

- Rust (1.70+)
- Cargo packages:
  - colored
  - dialoguer
  - serde
  - uuid
  - chrono

## Development :hammer_and_wrench:

```bash
# Run debug build
cargo run

# Run tests
cargo test

# Create release build
cargo build --release
```

## Roadmap :compass:

- Add tagging system
- Support multiple boards
- Add due dates
- Implement search

## Contributing :handshake:

PRs welcome! Please:

- Fork the repo
- Create a feature branch
- Submit a PR

## License :balance_scale:

MIT © DavidCks
