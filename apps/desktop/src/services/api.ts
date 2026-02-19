/**
 * Unified API - uses Tauri invoke when in desktop (local), fetch when in browser or cloud mode
 */
import {
  isTauri,
  apiCreateChat,
  apiListChats,
  apiGetMessages,
  apiSendMessage,
  apiSetFeedback,
} from '@idea-engine/shared';
import type { IdeaBundle } from '@idea-engine/shared';

type ApiContext = { useCloudBackend?: boolean; apiUrl?: string; apiKeys?: Record<string, string> };

async function invoke<T>(cmd: string, args?: object): Promise<T> {
  const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
  return tauriInvoke<T>(cmd, args as Record<string, unknown>);
}

function useCloud(ctx?: ApiContext) {
  return ctx?.useCloudBackend && ctx?.apiUrl;
}

export async function createChat(title: string, templateId?: string, ctx?: ApiContext): Promise<string> {
  if (isTauri() && !useCloud(ctx)) {
    return invoke<string>('create_chat', { input: { title, templateId: templateId ?? null } });
  }
  return apiCreateChat(title, templateId, ctx?.apiUrl);
}

export async function listChats(ctx?: ApiContext): Promise<Array<{ id: string; title: string }>> {
  if (isTauri() && !useCloud(ctx)) {
    return invoke<Array<{ id: string; title: string }>>('list_chats');
  }
  return apiListChats(ctx?.apiUrl);
}

export async function getChatMessages(chatId: string, ctx?: ApiContext) {
  if (isTauri() && !useCloud(ctx)) {
    return invoke('get_chat_messages', { chatId });
  }
  return apiGetMessages(chatId, ctx?.apiUrl);
}

export async function sendMessage(
  chatId: string,
  content: string,
  systemPrompt: string,
  providers: string[],
  apiKeys?: Record<string, string>,
  ctx?: ApiContext
): Promise<{ messageId: string; content: string; ideaBundles: IdeaBundle[] }> {
  if (isTauri() && !useCloud(ctx)) {
    return invoke('send_message', {
      input: { chatId, content, systemPrompt, providers, rubric: null },
    });
  }
  const keys = ctx?.apiKeys ?? apiKeys;
  if (!keys || Object.keys(keys).length === 0) {
    throw new Error('Add API keys in Settings to use the web/cloud version.');
  }
  return apiSendMessage(chatId, content, systemPrompt, providers, keys, ctx?.apiUrl);
}

export async function setFeedback(messageId: string, feedback: string, ctx?: ApiContext) {
  if (isTauri() && !useCloud(ctx)) {
    return invoke('set_feedback', { input: { messageId, feedback } });
  }
  return apiSetFeedback(messageId, feedback, ctx?.apiUrl);
}
