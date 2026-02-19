# Idea Engine - Implementation Output

## Repo File Tree

```
IdeaEngine/
├── .env.example
├── .gitignore
├── .npmrc
├── .prettierrc
├── Cargo.toml
├── DEPENDENCIES.md
├── OUTPUT.md
├── README.md
├── RUNBOOK.md
├── eslint.config.js
├── package.json
├── pnpm-workspace.yaml
├── rustfmt.toml
├── .github/
│   └── workflows/
│       ├── ci.yml
│       └── release.yml
├── apps/
│   └── desktop/
│       ├── index.html
│       ├── package.json
│       ├── tsconfig.json
│       ├── tsconfig.node.json
│       ├── vite.config.ts
│       ├── vitest.config.ts
│       ├── eslint.config.js
│       ├── public/
│       │   └── favicon.svg
│       ├── src/
│       │   ├── main.tsx
│       │   ├── App.tsx
│       │   ├── App.css
│       │   ├── index.css
│       │   ├── vite-env.d.ts
│       │   ├── components/
│       │   │   ├── ChatTimeline.tsx
│       │   │   ├── ChatTimeline.css
│       │   │   ├── IdeaCard.tsx
│       │   │   ├── IdeaCard.css
│       │   │   ├── ModelSelector.tsx
│       │   │   ├── ModelSelector.css
│       │   │   ├── Settings.tsx
│       │   │   ├── Settings.css
│       │   │   ├── TemplateSidebar.tsx
│       │   │   └── TemplateSidebar.css
│       │   └── store/
│       │       └── chatStore.ts
│       └── src-tauri/
│           ├── Cargo.toml
│           ├── build.rs
│           ├── tauri.conf.json
│           ├── capabilities/
│           │   └── default.json
│           ├── icons/
│           │   ├── 32x32.png
│           │   ├── 128x128.png
│           │   ├── 128x128@2x.png
│           │   └── README.md
│           └── src/
│               ├── main.rs
│               ├── lib.rs
│               └── ipc.rs
├── crates/
│   └── core/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── schema.rs
│           ├── storage.rs
│           ├── orchestrator.rs
│           ├── ranker.rs
│           ├── eval.rs
│           ├── bin/
│           │   └── eval.rs
│           └── adapters/
│               ├── mod.rs
│               ├── openai.rs
│               ├── anthropic.rs
│               └── gemini.rs
├── packages/
│   └── shared/
│       ├── package.json
│       ├── tsconfig.json
│       ├── tsup.config.ts
│       ├── vitest.config.ts
│       ├── eslint.config.js
│       └── src/
│           ├── index.ts
│           ├── index.test.ts
│           ├── types.ts
│           ├── schemas.ts
│           └── templates/
│               └── youtube-playlist.ts
└── scripts/
    └── gen-icon.js
```

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
OpenAI Adapter      Anthropic Adapter   Gemini Adapter
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

## Key Code Files

| File | Purpose |
|------|---------|
| `crates/core/src/orchestrator.rs` | Fan-out, timeout, retry logic |
| `crates/core/src/adapters/*.rs` | OpenAI, Anthropic, Gemini adapters |
| `crates/core/src/schema.rs` | IdeaBundle, AIResponse types |
| `crates/core/src/storage.rs` | SQLite persistence |
| `crates/core/src/ranker.rs` | Rubric scoring, dedupe |
| `apps/desktop/src-tauri/src/ipc.rs` | Typed Tauri commands |
| `packages/shared/src/schemas.ts` | Zod validation |
| `packages/shared/src/types.ts` | Shared TS types |
| `apps/desktop/src/components/ChatTimeline.tsx` | Chat UI |
| `packages/shared/src/templates/youtube-playlist.ts` | YouTube template |

## GitHub Actions

- **ci.yml**: Lint, test, build on push/PR to main
- **release.yml**: Build Windows, macOS, Linux bundles on tag push `v*`

See `.github/workflows/` for full YAML.

## Runbook Summary

- **Setup**: Node 20, pnpm 9, Rust stable, Tauri deps
- **Dev**: `pnpm dev`
- **Build**: `pnpm build`
- **Release**: `pnpm tauri build` (in apps/desktop)
- **Eval**: `pnpm eval -- --db path --recipe id --problems id1,id2`
