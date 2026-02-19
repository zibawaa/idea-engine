# Deploy Idea Engine (Web + Connect Desktop)

## Architecture

- **Frontend** (React) → Netlify (free)
- **Backend API** (Node.js) → Render (free)
- **Desktop** (Tauri) → Can connect to the deployed API via Settings

## 1. Deploy API to Render

1. Push repo to GitHub
2. Go to [render.com](https://render.com) → New → Web Service
3. Connect your GitHub repo
4. Settings:
   - **Build Command:** `pnpm install`
   - **Start Command:** `node apps/api/server.js`
   - **Root Directory:** (leave default)
5. Deploy. Note your URL, e.g. `https://idea-engine-api.onrender.com`

**Note:** Render free tier spins down after 15 min idle. First request may take ~30s.

## 2. Deploy Frontend to Netlify

1. Go to [netlify.com](https://netlify.com) → Add new site → Import from Git
2. Connect your GitHub repo
3. Build settings:
   - **Build command:** `pnpm install && pnpm --filter @idea-engine/shared build && pnpm --filter desktop build`
   - **Publish directory:** `apps/desktop/dist`
4. **Environment variables:** Add `VITE_API_URL` = your Render API URL (e.g. `https://idea-engine-api.onrender.com`)
5. Deploy

## 3. Connect Desktop to Cloud

1. Open Idea Engine desktop app
2. Click **Settings**
3. Enable **Use cloud backend**
4. Enter your Render API URL
5. Add API keys (stored in OS keychain, sent to API when using cloud)
6. Chats and messages will sync through the cloud backend

## Local API (for development)

```bash
pnpm api:dev   # API on http://localhost:3000
```

Then run the web app (Vite) with `VITE_API_URL=http://localhost:3000` or use the desktop with cloud mode + `http://localhost:3000`.
