# Idea Engine Runbook

## Setup

1. **Prerequisites**
   - Node.js 20 LTS
   - pnpm 9: `npm install -g pnpm`
   - Rust stable: `rustup default stable`
   - [Tauri prerequisites](https://v2.tauri.app/start/prerequisites) for your OS

2. **Install**
   ```bash
   git clone <repo-url>
   cd IdeaEngine
   pnpm install
   ```

3. **Icons (before first Tauri build)**
   ```bash
   pnpm icon
   # Or: pnpm tauri icon path/to/your-1024x1024.png
   ```

4. **API keys**
   - Copy `.env.example` to `.env` (optional, for env fallback)
   - Or use Settings in-app to store in OS keychain

## Dev

```bash
pnpm dev
```

Starts Vite + Tauri. Hot reload for frontend.

## Build

```bash
pnpm build
```

Builds React app and Tauri production bundle.

## Release

```bash
cd apps/desktop
pnpm tauri build
```

Outputs:
- Windows: `target/x86_64-pc-windows-msvc/release/bundle/msi/`
- macOS: `target/aarch64-apple-darwin/release/bundle/dmg/`
- Linux: `target/x86_64-unknown-linux-gnu/release/bundle/appimage/`

## Lint & Test

```bash
pnpm lint
pnpm test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --workspace
```

## Eval Runner

```bash
cargo run -p idea-engine-core -- eval --db path/to/db.sqlite --recipe <id> --problems <id1,id2>
```

(Placeholder - full impl in `crates/core/src/eval.rs`)
