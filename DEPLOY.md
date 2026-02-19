# Deploy Idea Engine (Web + Connect Desktop)

## Architecture

- **Frontend** (React) → Netlify (free)
- **Backend API** (Node.js) → Render (free)
- **Desktop** (Tauri) → Can connect to the deployed API via Settings

## Current Deployment

- **Frontend (live):** https://fastidious-biscotti-b09665.netlify.app
- **API:** Deploy to Render (see below), then add `VITE_API_URL` in Netlify env vars and redeploy.

## 1. Deploy API to Render

1. Push repo to GitHub:
   ```bash
   git remote add origin https://github.com/YOUR_USERNAME/idea-engine.git
   git push -u origin master
   ```
2. Go to [dashboard.render.com](https://dashboard.render.com) → New → Web Service
3. Connect GitHub and select your repo, or paste the public repo URL
4. Settings (or use `render.yaml` blueprint):
   - **Build Command:** `pnpm install`
   - **Start Command:** `node apps/api/server.js`
   - **Root Directory:** (leave default)
5. Deploy. Note your URL, e.g. `https://idea-engine-api.onrender.com`

**Note:** Render free tier spins down after 15 min idle. First request may take ~30s.

### Deploy Frontend (Static Site) to Render

If deploying the web frontend to Render instead of Netlify:
- **Build Command:** `pnpm install && pnpm --filter @idea-engine/shared build && pnpm --filter desktop build`
- **Publish Directory:** `apps/desktop/dist`
- **Root Directory:** (leave default) — must build from repo root so shared package is available

## 2. Connect Frontend to API

1. Go to [Netlify → Site settings → Environment variables](https://app.netlify.com/sites/fastidious-biscotti-b09665/configuration/env)
2. Add `VITE_API_URL` = your Render API URL (e.g. `https://idea-engine-api.onrender.com`)
3. Trigger a redeploy (Deploys → Trigger deploy)

## 3. Deploy Frontend (or redeploy)

```bash
cd apps/desktop
npx netlify-cli deploy --prod --dir=dist --create-site
```

Or: Netlify → Deploys → Trigger deploy (if already linked).

## 4. Connect Desktop to Cloud

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
