# Idea Engine - Exact Package Dependencies

## Root (package.json)

```json
{
  "devDependencies": {
    "@eslint/js": "^9.15.0",
    "eslint": "^9.15.0",
    "eslint-plugin-react-hooks": "^5.0.0",
    "eslint-plugin-react-refresh": "^0.4.14",
    "typescript-eslint": "^8.15.0"
  }
}
```

## packages/shared

```json
{
  "dependencies": {
    "zod": "^3.24.1"
  },
  "devDependencies": {
    "@types/node": "^22.10.1",
    "tsup": "^8.3.5",
    "typescript": "~5.7.2",
    "vitest": "^2.1.6"
  }
}
```

## apps/desktop

```json
{
  "dependencies": {
    "@idea-engine/shared": "workspace:*",
    "@tanstack/react-query": "^5.62.0",
    "@tauri-apps/api": "^2.0.0",
    "@tauri-apps/plugin-shell": "^2.0.0",
    "ai": "^4.0.0",
    "@ai-sdk/openai": "^1.0.0",
    "@ai-sdk/anthropic": "^1.0.0",
    "@ai-sdk/google": "^1.0.0",
    "react": "^18.3.1",
    "react-dom": "^18.3.1",
    "zustand": "^5.0.2"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0",
    "@types/react": "^18.3.12",
    "@types/react-dom": "^18.3.1",
    "@vitejs/plugin-react": "^4.3.4",
    "eslint": "^9.0.0",
    "eslint-plugin-react-hooks": "^5.0.0",
    "eslint-plugin-react-refresh": "^0.4.14",
    "prettier": "^3.4.2",
    "typescript": "~5.7.2",
    "vite": "^5.4.11",
    "vitest": "^2.1.6",
    "jsdom": "^25.0.0"
  }
}
```

## crates/core (Cargo.toml)

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
uuid = { version = "1", features = ["v4", "serde"] }
rusqlite = { version = "0.32", features = ["bundled"] }
keyring = "2.0"
async-trait = "0.1"
```

## apps/desktop/src-tauri (Cargo.toml)

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
idea-engine-core = { path = "../../../crates/core" }
keyring = "2.0"
uuid = { version = "1", features = ["v4"] }
```
