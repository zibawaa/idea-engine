/**
 * Idea Engine API - Web backend for Netlify/Render deployment
 * Mirrors Tauri IPC so the same React app works in browser and desktop
 */
import express from 'express';
import cors from 'cors';
import { v4 as uuidv4 } from 'uuid';
import OpenAI from 'openai';
import Anthropic from '@anthropic-ai/sdk';
import { GoogleGenerativeAI } from '@google/generative-ai';

const app = express();
app.use(cors());
app.use(express.json({ limit: '1mb' }));

// In-memory store (resets on restart - use DB for production)
const chats = new Map();
const messages = new Map();

const now = () => Math.floor(Date.now() / 1000).toString();

// OpenAI
async function callOpenAI(apiKey, systemPrompt, userPrompt, model = 'gpt-4o-mini') {
  const openai = new OpenAI({ apiKey });
  const res = await openai.chat.completions.create({
    model,
    messages: [
      { role: 'system', content: systemPrompt + '\n\nRespond with valid JSON only.' },
      { role: 'user', content: userPrompt },
    ],
    response_format: { type: 'json_object' },
    temperature: 0.7,
  });
  return JSON.parse(res.choices[0].message.content);
}

// Anthropic
async function callAnthropic(apiKey, systemPrompt, userPrompt, model = 'claude-3-5-haiku-20241022') {
  const anthropic = new Anthropic({ apiKey });
  const res = await anthropic.messages.create({
    model,
    max_tokens: 4096,
    system: systemPrompt + '\n\nRespond with valid JSON only.',
    messages: [{ role: 'user', content: userPrompt }],
    temperature: 0.7,
  });
  const text = res.content[0].text;
  const jsonMatch = text.match(/\{[\s\S]*\}/);
  return JSON.parse(jsonMatch ? jsonMatch[0] : text);
}

// Gemini
async function callGemini(apiKey, systemPrompt, userPrompt, model = 'gemini-1.5-flash') {
  const genai = new GoogleGenerativeAI(apiKey);
  const m = genai.getGenerativeModel({ model, generationConfig: { responseMimeType: 'application/json' } });
  const res = await m.generateContent(systemPrompt + '\n\n---\n\n' + userPrompt);
  return JSON.parse(res.response.text());
}

function normalizeResponse(raw) {
  return {
    ideas: raw.ideas || [],
    stepPlan: raw.step_plan || raw.stepPlan || [],
    risks: raw.risks || [],
    dependencies: raw.dependencies || [],
    effort: raw.effort || { time: 'Unknown' },
    nextActions: raw.next_actions || raw.nextActions || [],
  };
}

// Routes
app.post('/api/chats', (req, res) => {
  const { title, templateId } = req.body;
  const id = uuidv4();
  const t = now();
  chats.set(id, { id, title: title || 'New Chat', templateId: templateId || null, createdAt: t, updatedAt: t });
  res.json(id);
});

app.get('/api/chats', (req, res) => {
  const rows = [...chats.values()].sort((a, b) => Number(b.updatedAt) - Number(a.updatedAt));
  res.json(rows.map(r => ({ id: r.id, title: r.title, templateId: r.templateId, createdAt: r.createdAt, updatedAt: r.updatedAt })));
});

app.get('/api/chats/:id/messages', (req, res) => {
  const rows = [...messages.values()].filter(m => m.chatId === req.params.id).sort((a, b) => Number(a.createdAt) - Number(b.createdAt));
  res.json(rows.map(r => ({
    id: r.id, chatId: r.chatId, role: r.role, content: r.content,
    ideaBundles: r.ideaBundlesJson ? JSON.parse(r.ideaBundlesJson) : null,
    feedback: r.feedback, createdAt: r.createdAt,
  })));
});

app.post('/api/chats/:id/messages', async (req, res) => {
  const chatId = req.params.id;
  const { content, systemPrompt, providers, apiKeys } = req.body;
  const msgId = uuidv4();
  const asstId = uuidv4();
  const t = now();

  messages.set(msgId, { id: msgId, chatId, role: 'user', content, ideaBundlesJson: null, feedback: null, createdAt: t });
  const c = chats.get(chatId);
  if (c) { c.updatedAt = t; chats.set(chatId, c); }

  const bundles = [];
  const errors = [];

  for (const p of providers || ['openai']) {
    try {
      const key = apiKeys?.[p];
      if (!key) { errors.push(`${p}: Missing API key`); continue; }
      let raw;
      if (p === 'openai') raw = await callOpenAI(key, systemPrompt, content);
      else if (p === 'anthropic') raw = await callAnthropic(key, systemPrompt, content);
      else if (p === 'gemini') raw = await callGemini(key, systemPrompt, content);
      else continue;
      const norm = normalizeResponse(raw);
      bundles.push({ id: uuidv4(), provider: p, model: p, ...norm, createdAt: t });
    } catch (e) {
      errors.push(`${p}: ${e.message}`);
    }
  }

  const assistantContent = bundles.length > 0
    ? `Generated ${bundles.length} idea bundle(s).`
    : `No ideas generated. ${errors.join('; ')}`;

  messages.set(asstId, { id: asstId, chatId, role: 'assistant', content: assistantContent, ideaBundlesJson: JSON.stringify(bundles), feedback: null, createdAt: t });
  if (c) { c.updatedAt = t; chats.set(chatId, c); }

  res.json({ messageId: asstId, content: assistantContent, ideaBundles: bundles, errors });
});

app.post('/api/messages/:id/feedback', (req, res) => {
  const m = messages.get(req.params.id);
  if (m) { m.feedback = req.body.feedback; messages.set(req.params.id, m); }
  res.json({ ok: true });
});

app.get('/api/health', (req, res) => res.json({ ok: true }));

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => console.log(`Idea Engine API on port ${PORT}`));
