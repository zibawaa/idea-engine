# Idea Engine

Cross-platform desktop app that routes one user problem to multiple AI providers (OpenAI, Anthropic, Google Gemini), then merges, scores, and returns the best ideas as actionable plans.

## System Flow Diagram

```
User
|
v
Desktop UI
  - chat input
  - template picker
  - model selector
|
v
Prompt Builder
  - problem framing
  - constraints
  - rubric
  - few shot examples
|
v
Orchestrator
  - fan out requests
  - retries
  - timeouts
|
+-------------------+-------------------+-------------------+
|                   |                   |                   |
v                   v                   v
OpenAI Adapter      Anthropic Adapter  Gemini Adapter
|                   |                   |
v                   v                   v
Model Responses (JSON)
|                   |                   |
+---------+---------+---------+---------+
|
v
Normalizer
  - validate schema
  - convert to IdeaBundle
|
v
Ranker
  - apply rubric
  - dedupe
  - pick top ideas
|
v
Presenter
  - best ideas
  - step plans
  - risks
  - next actions
|
v
Local Store (SQLite)
  - chats
  - prompt recipes
  - scores
  - feedback
  - eval datasets
```

## Tech Stack

| Layer | Technology | Version |
|-------|------------|---------|
| Desktop shell | Tauri | 2.x |
| Backend core | Rust | stable |
| Frontend | React, TypeScript, Vite | 18, 5.x, 5 |
| State | TanStack Query, Zustand | 5.x, 5.x |
| Persistence | SQLite (Rust) | rusqlite 0.32 |
| AI orchestration | Vercel AI SDK core | ai 4.x |
| Provider SDKs | openai, @anthropic-ai/sdk, @google/genai | latest |

## Runbook

### Setup

1. **Prerequisites**
   - Node.js 20 LTS
   - pnpm 9 (`npm install -g pnpm`)
   - Rust stable (`rustup default stable`)
   - Tauri system deps: https://v2.tauri.app/start/prerequisites

2. **Clone and install**
   ```bash
   git clone <repo-url>
   cd IdeaEngine
   pnpm install
   ```

3. **API keys (optional for dev)**
   - Copy `.env.example` to `.env`
   - Add keys: `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `GOOGLE_GENERATIVE_AI_API_KEY`
   - Or use Settings in-app to store in OS keychain

### Dev

```bash
pnpm dev
```

Starts Vite dev server + Tauri window. Hot reload for frontend.

### Build

```bash
pnpm build
```

Builds React app and Tauri production bundle.

### Release

```bash
pnpm tauri build
```

Outputs platform-specific installers in `apps/desktop/src-tauri/target/*/release/bundle/`.

For all platforms (from CI):
- Windows: `.msi`
- macOS: `.dmg`
- Linux: `.AppImage`, `.deb`

### Lint & Test

```bash
pnpm lint
pnpm test
cargo fmt --check
cargo clippy --all-targets
cargo test --workspace
```

## Web Deployment (Netlify + Render)

Deploy the web version for free. See [DEPLOY.md](DEPLOY.md).

- **Frontend** → Netlify
- **Backend API** → Render (free tier)
- **Desktop** → Settings → "Use cloud backend" + API URL to connect

## Repo Structure

```
IdeaEngine/
├── apps/
│   ├── api/               # Node.js backend (Render)
│   └── desktop/           # Tauri + React UI
│       ├── src/
│       │   ├── components/
│       │   ├── store/
│       │   └── App.tsx
│       └── src-tauri/     # Rust backend
├── crates/
│   └── core/              # Domain logic
│       ├── src/
│       │   ├── adapters/   # OpenAI, Anthropic, Gemini
│       │   ├── orchestrator.rs
│       │   ├── ranker.rs
│       │   ├── schema.rs
│       │   ├── storage.rs
│       │   └── eval.rs
│       └── Cargo.toml
├── packages/
│   └── shared/            # TS types, schemas, templates
│       └── src/
├── scripts/
│   └── gen-icon.js
├── .github/workflows/
│   ├── ci.yml
│   └── release.yml
├── Cargo.toml
├── package.json
└── pnpm-workspace.yaml
```

## Template: YouTube Playlist Auto Translate

Inputs: playlist URL, target language, output format (SRT/VTT).

Plan uses official YouTube Data API for video IDs and caption access. If captions missing: propose speech-to-text → translation → SRT/VTT generation. No scraping.
