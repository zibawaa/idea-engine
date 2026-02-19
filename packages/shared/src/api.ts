/**
 * API client - used when running in browser (not Tauri)
 * Set VITE_API_URL for the deployed backend
 */

export const getApiUrl = (override?: string) =>
  override ||
  (typeof import.meta !== 'undefined' && (import.meta as { env?: { VITE_API_URL?: string } }).env?.VITE_API_URL) ||
  'http://localhost:3000';

export async function apiCreateChat(title: string, templateId?: string, apiUrl?: string): Promise<string> {
  const res = await fetch(`${getApiUrl(apiUrl)}/api/chats`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ title, templateId }),
  });
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

export async function apiListChats(apiUrl?: string): Promise<Array<{ id: string; title: string }>> {
  const res = await fetch(`${getApiUrl(apiUrl)}/api/chats`);
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

export async function apiGetMessages(chatId: string, apiUrl?: string) {
  const res = await fetch(`${getApiUrl(apiUrl)}/api/chats/${chatId}/messages`);
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

export async function apiSendMessage(
  chatId: string,
  content: string,
  systemPrompt: string,
  providers: string[],
  apiKeys: Record<string, string>,
  apiUrl?: string
) {
  const res = await fetch(`${getApiUrl(apiUrl)}/api/chats/${chatId}/messages`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ content, systemPrompt, providers, apiKeys }),
  });
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

export async function apiSetFeedback(messageId: string, feedback: string, apiUrl?: string) {
  const res = await fetch(`${getApiUrl(apiUrl)}/api/messages/${messageId}/feedback`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ feedback }),
  });
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

export const isTauri = () => typeof window !== 'undefined' && '__TAURI__' in window;
