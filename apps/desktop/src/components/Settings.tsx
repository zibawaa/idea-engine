import { useState } from 'react';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { isTauri } from '@idea-engine/shared';
import { useChatStore } from '../store/chatStore';
import './Settings.css';

async function getTauriKeys() {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<{ openai: boolean; anthropic: boolean; gemini: boolean }>('get_api_keys');
}

async function setTauriKey(provider: string, key: string) {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke('set_api_key', { input: { provider, key } });
}

interface SettingsProps {
  open: boolean;
  onClose: () => void;
}

export function Settings({ open, onClose }: SettingsProps) {
  const [openaiKey, setOpenaiKey] = useState('');
  const [anthropicKey, setAnthropicKey] = useState('');
  const [geminiKey, setGeminiKey] = useState('');
  const queryClient = useQueryClient();
  const { apiKeys, setApiKey, useCloudBackend, setUseCloudBackend, apiUrl, setApiUrl } = useChatStore();

  const { data: tauriKeys } = useQuery({
    queryKey: ['api-keys'],
    queryFn: getTauriKeys,
    enabled: open && isTauri(),
  });

  const hasKey = (p: string) => (isTauri() ? (tauriKeys as Record<string, boolean>)?.[p] : !!apiKeys[p]);

  const saveKey = async (provider: string, key: string) => {
    if (!key.trim()) return;
    if (isTauri()) {
      await setTauriKey(provider, key);
      queryClient.invalidateQueries({ queryKey: ['api-keys'] });
    } else {
      setApiKey(provider, key);
    }
  };

  if (!open) return null;

  return (
    <div className="settings-overlay" onClick={onClose}>
      <div className="settings-modal" onClick={(e) => e.stopPropagation()}>
        <h2>API Keys</h2>
        {isTauri() && (
          <>
            <div className="settings-row">
              <label>Use cloud backend</label>
              <input
                type="checkbox"
                checked={useCloudBackend}
                onChange={(e) => setUseCloudBackend(e.target.checked)}
              />
            </div>
            {useCloudBackend && (
              <div className="settings-row">
                <label>API URL</label>
                <input
                  type="url"
                  placeholder="https://your-api.onrender.com"
                  value={apiUrl}
                  onChange={(e) => setApiUrl(e.target.value)}
                />
              </div>
            )}
          </>
        )}
        <p className="settings-hint">
          {isTauri() ? 'Stored in OS keychain. Never committed.' : 'Stored locally in browser. Required for web version.'}
        </p>
        <div className="settings-row">
          <label>OpenAI</label>
          <input
            type="password"
            placeholder={hasKey('openai') ? '••••••••' : 'sk-...'}
            value={openaiKey}
            onChange={(e) => setOpenaiKey(e.target.value)}
          />
          <button onClick={() => saveKey('openai', openaiKey)}>Save</button>
        </div>
        <div className="settings-row">
          <label>Anthropic</label>
          <input
            type="password"
            placeholder={hasKey('anthropic') ? '••••••••' : 'sk-ant-...'}
            value={anthropicKey}
            onChange={(e) => setAnthropicKey(e.target.value)}
          />
          <button onClick={() => saveKey('anthropic', anthropicKey)}>Save</button>
        </div>
        <div className="settings-row">
          <label>Google Gemini</label>
          <input
            type="password"
            placeholder={hasKey('gemini') ? '••••••••' : 'AIza...'}
            value={geminiKey}
            onChange={(e) => setGeminiKey(e.target.value)}
          />
          <button onClick={() => saveKey('gemini', geminiKey)}>Save</button>
        </div>
        <button className="settings-close" onClick={onClose}>Close</button>
      </div>
    </div>
  );
}
