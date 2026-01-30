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

```bash
# Clone the repository
git clone https://github.com/yourusername/opencode-monitor
cd opencode-monitor

# Install dependencies
npm install

# Build Rust backend
cd src-tauri
cargo build

# Run in dev mode
cd ..
npm run tauri dev
```

## Configuration

### OpenCode Server

Start OpenCode server:
```bash
opencode serve --port 4096 --hostname 0.0.0.0
```

### Pi Configuration

Default Pi settings:
- Model: `gpt-5.2-codex`
- Thinking: `xhigh`
- Provider: `github-copilot`

Configure via UI or environment:
```bash
export GITHUB_TOKEN=your_token
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
