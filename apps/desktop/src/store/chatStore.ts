import { create } from 'zustand';
import { persist } from 'zustand/middleware';

const WEB_KEYS = 'idea-engine-api-keys';

interface ChatState {
  currentChatId: string | null;
  setCurrentChatId: (id: string | null) => void;
  selectedProviders: string[];
  setSelectedProviders: (providers: string[]) => void;
  selectedTemplate: string | null;
  setSelectedTemplate: (id: string | null) => void;
  apiKeys: Record<string, string>;
  setApiKey: (provider: string, key: string) => void;
  useCloudBackend: boolean;
  setUseCloudBackend: (v: boolean) => void;
  apiUrl: string;
  setApiUrl: (url: string) => void;
}

export const useChatStore = create<ChatState>()(
  persist(
    (set) => ({
      currentChatId: null,
      setCurrentChatId: (id) => set({ currentChatId: id }),
      selectedProviders: ['openai', 'anthropic', 'gemini'],
      setSelectedProviders: (providers) => set({ selectedProviders: providers }),
      selectedTemplate: null,
      setSelectedTemplate: (id) => set({ selectedTemplate: id }),
      apiKeys: {},
      setApiKey: (provider, key) =>
        set((s) => ({ apiKeys: { ...s.apiKeys, [provider]: key } })),
      useCloudBackend: false,
      setUseCloudBackend: (v) => set({ useCloudBackend: v }),
      apiUrl: '',
      setApiUrl: (url) => set({ apiUrl: url }),
    }),
    { name: WEB_KEYS, partialize: (s) => ({ apiKeys: s.apiKeys, useCloudBackend: s.useCloudBackend, apiUrl: s.apiUrl }) }
  )
);
