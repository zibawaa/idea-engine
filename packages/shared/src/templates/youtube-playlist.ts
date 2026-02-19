/**
 * YouTube Playlist Auto Translate - Example workflow template
 * Inputs: playlist URL, target language, output format
 * Plan uses official YouTube Data API for video IDs and caption access.
 * If captions missing: propose speech-to-text → translation → SRT/VTT generation.
 */

import type { PromptRecipe } from '../types';

export const YOUTUBE_PLAYLIST_TEMPLATE: PromptRecipe = {
  id: 'youtube-playlist-auto-translate',
  name: 'YouTube Playlist Auto Translate',
  systemPrompt: `You are an idea engine specializing in video localization workflows.
Generate actionable plans as structured JSON.
Output schema: ideas (array of {title, description, rationale?}), step_plan (array of {order, action, details?}), risks (array of {description, severity, mitigation?}), dependencies (array of strings), effort ({time, cost?, complexity?}), next_actions (array of {action, priority}).

Constraints for this template:
- Use ONLY the official YouTube Data API v3 for:
  - Fetching playlist video IDs
  - Accessing captions/timedtext
- Do NOT propose web scraping or unofficial APIs.
- If captions are missing for a video: propose speech-to-text (e.g., Whisper API) → translation → SRT/VTT generation.
- Output format options: SRT, VTT, or both.
- Consider: API quotas, batch processing, cost estimates.`,
  userPromptTemplate: `Create a plan to auto-translate a YouTube playlist.

Inputs:
- Playlist URL: {{playlistUrl}}
- Target language: {{targetLanguage}}
- Output format: {{outputFormat}} (SRT, VTT, or both)

Provide a step-by-step plan using the YouTube Data API and caption/translation services.`,
  rubric: {
    novelty: 1,
    feasibility: 2,
    cost: 1.5,
    time: 1,
    risk: 1.5,
    clarity: 2,
  },
  fewShotExamples: [
    'Example: Step 1 - Use YouTube Data API playlists.list to get video IDs from playlist URL.',
    'Example: Step 2 - For each video, use captions.list to check for existing captions.',
    'Example: Step 3 - If no captions: use speech-to-text API (e.g., Whisper) on audio, then translate with translation API.',
  ],
  createdAt: new Date().toISOString(),
};
