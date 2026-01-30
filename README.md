# OpenCodeMonitor

A Tauri-based dashboard for managing OpenCode sessions and Pi coding agents.

## Overview

Forked from [CodexMonitor](https://github.com/dimillian/CodexMonitor) and adapted for:
- **OpenCode** - AI coding agent with REST API (localhost:4096)
- **Pi** - Coding agent with gpt-5.2-codex and custom Copilot prompts

## Features

### OpenCode Integration
- List/create/delete sessions
- Send messages and view responses
- View file diffs
- File browsing and search
- Git integration

### Pi Agent Integration
- Configure Pi settings (model, thinking level, system prompt)
- Run Pi sessions with custom prompts
- Real-time output streaming
- Multiple concurrent sessions

## Installation

### Prerequisites

- **Rust** 1.70+ (`rustup` recommended)
- **Node.js** 18+
- **Tauri CLI**: `cargo install tauri-cli`
- **OpenCode** (optional): For local server
- **Pi CLI** (optional): For coding agent

### Platform-Specific Setup

#### Windows
```powershell
# Install Rust (if not installed)
winget install Rustlang.Rust.MSVC

# Install Node.js
winget install OpenJS.NodeJS.LTS

# Install Tauri CLI
cargo install tauri-cli

# Install Pi CLI (GitHub Copilot)
# Download from: https://github.com/ok-norlaker/pi/releases
# Add to PATH

# Clone and build
git clone https://github.com/orkuhh/opencode-monitor
cd opencode-monitor
npm install
cargo install tauri-cli --version 2
cargo tauri build --bundles app
```

#### macOS
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (via Homebrew)
brew install node

# Install Tauri CLI
cargo install tauri-cli

# Clone and build
git clone https://github.com/orkuhh/opencode-monitor
cd opencode-monitor
npm install
cargo tauri build --bundles app
```

#### Linux (Ubuntu/Debian)
```bash
# Install dependencies
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev cargo nodejs

# Install Tauri CLI
cargo install tauri-cli

# Clone and build
git clone https://github.com/orkuhh/opencode-monitor
cd opencode-monitor
npm install
cargo tauri build --bundles app
```

### Development Mode

```bash
# Run in dev mode (frontend + backend)
npm run tauri dev
```

## Configuration

### OpenCode Server

Start OpenCode server:
```bash
# Linux/macOS
opencode serve --port 4096 --hostname 0.0.0.0

# Windows
opencode.exe serve --port 4096 --hostname 0.0.0.0
```

### Pi Configuration

Default Pi settings:
- Model: `gpt-5.2-codex`
- Thinking: `xhigh`
- Provider: `github-copilot`

Configure via UI or environment:
```bash
# Linux/macOS
export GITHUB_TOKEN=your_token

# Windows (PowerShell)
$env:GITHUB_TOKEN="your_token"
```

## API Reference

### OpenCode Commands

| Command | Description |
|---------|-------------|
| `opencode_health` | Check server health |
| `opencode_list_sessions` | List all sessions |
| `opencode_create_session` | Create new session |
| `opencode_send_message` | Send message to session |
| `opencode_get_messages` | Get session messages |
| `opencode_get_diffs` | Get file diffs |
| `opencode_abort_session` | Abort running session |
| `opencode_delete_session` | Delete session |
| `opencode_search_files` | Search files |
| `opencode_read_file` | Read file content |
| `opencode_list_files` | List directory |

### Pi Commands

| Command | Description |
|---------|-------------|
| `pi_list_models` | List available models |
| `pi_get_config` | Get current Pi config |
| `pi_update_config` | Update Pi config |
| `pi_run_session` | Run Pi session |
| `pi_wait_session` | Wait for session to complete |
| `pi_kill_session` | Kill running session |
| `pi_get_output` | Get session output |

## Development

### Project Structure

```
src-tauri/src/
├── lib.rs              # Main Tauri app
├── opencode/           # OpenCode client & commands
│   ├── mod.rs
│   ├── opencode.rs     # HTTP client
│   └── commands.rs     # Tauri commands
├── pi/                 # Pi agent integration
│   ├── mod.rs
│   ├── pi.rs           # Pi session manager
│   └── commands.rs     # Tauri commands
└── ...

src/
└── features/           # Frontend React components
```

### Adding New Commands

1. Add function to `opencode.rs` or `pi.rs`
2. Create command wrapper in `commands.rs`
3. Export in `mod.rs`
4. Register in `lib.rs` invoke_handler

## License

MIT - Forked from CodexMonitor

## Platform Notes

### Windows
- Full support with Windows-specific dependencies
- Pi CLI must be installed separately (add to PATH)
- Dictation/voice features disabled

### macOS
- Full support including:
  - Finder app icon integration
  - Voice dictation (whisper-rs)
  - macOS-native window management

### Linux
- Full support
- May require additional audio dependencies for dictation
