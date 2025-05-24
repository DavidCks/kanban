#!/usr/bin/env bash
set -e

REPO="https://github.com/<user>/<repo>.git"
APP_NAME="kanban"
INSTALL_DIR="/usr/local/bin"

echo "📦 Installing $APP_NAME from $REPO"

# Create temp dir
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

cd $TMP_DIR

echo "📥 Cloning repo..."
git clone --depth 1 "$REPO" .
echo "🔧 Building binary..."
cargo build --release

echo "🚀 Installing to $INSTALL_DIR..."
sudo cp "target/release/$APP_NAME" "$INSTALL_DIR"

echo "✅ Installed $APP_NAME to $INSTALL_DIR"
"$APP_NAME" --help || echo "Run '$APP_NAME' to get started!"
