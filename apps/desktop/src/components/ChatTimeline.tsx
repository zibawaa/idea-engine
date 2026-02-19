import { useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { useChatStore } from '../store/chatStore';
import { IdeaCard } from './IdeaCard';
import { ModelSelector } from './ModelSelector';
import { getChatMessages, sendMessage, setFeedback } from '../services/api';
import { useApiContext } from '../hooks/useApiContext';
import type { IdeaBundle } from '@idea-engine/shared';
import './ChatTimeline.css';

const YOUTUBE_SYSTEM_PROMPT = `You are an idea engine. Generate actionable plans as structured JSON.
Output schema: ideas (array of {title, description, rationale?}), step_plan (array of {order, action, details?}), risks (array of {description, severity, mitigation?}), dependencies (array of strings), effort ({time, cost?, complexity?}), next_actions (array of {action, priority}).
For YouTube playlist auto-translate: use official YouTube Data API for video IDs and caption access. If captions missing, propose speech-to-text then translation, then SRT/VTT generation. Do not ship scraping.`;

interface Message {
  id: string;
  chatId: string;
  role: string;
  content: string;
  ideaBundles?: IdeaBundle[];
  feedback?: string;
  createdAt: string;
}

interface ChatTimelineProps {
  chatId: string | null;
}

export function ChatTimeline({ chatId }: ChatTimelineProps) {
  const queryClient = useQueryClient();
  const [input, setInput] = useState('');
  const { selectedProviders, setSelectedProviders } = useChatStore();
  const apiCtx = useApiContext();

  const { data: messages = [] } = useQuery({
    queryKey: ['messages', chatId, apiCtx.useCloudBackend, apiCtx.apiUrl],
    queryFn: () => getChatMessages(chatId ?? '', apiCtx),
    enabled: !!chatId,
  });

  const sendMutation = useMutation({
    mutationFn: (content: string) =>
      sendMessage(chatId!, content, YOUTUBE_SYSTEM_PROMPT, selectedProviders, undefined, apiCtx),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['messages', chatId] });
    },
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim() || !chatId) return;
    sendMutation.mutate(input.trim());
    setInput('');
  };

  const handleFeedback = (messageId: string, feedback: string) => {
    setFeedback(messageId, feedback, apiCtx).then(() => {
      queryClient.invalidateQueries({ queryKey: ['messages', chatId] });
    });
  };

  if (!chatId) {
    return (
      <div className="chat-empty">
        <p>Select a chat or create a new one to get started.</p>
      </div>
    );
  }

  return (
    <div className="chat-timeline">
      <ModelSelector
        selected={selectedProviders}
        onChange={setSelectedProviders}
      />
      <div className="messages">
        {messages.map((m: Message) => (
          <div key={m.id} className={`message message-${m.role}`}>
            <div className="message-content">{m.content}</div>
            {m.ideaBundles && m.ideaBundles.length > 0 && (
              <div className="idea-bundles">
                {m.ideaBundles.map((b: IdeaBundle) => (
                  <IdeaCard key={b.id} bundle={b} />
                ))}
              </div>
            )}
            {m.role === 'assistant' && (
              <div className="feedback-buttons">
                <button onClick={() => handleFeedback(m.id, 'helpful')}>👍 Helpful</button>
                <button onClick={() => handleFeedback(m.id, 'not_helpful')}>👎 Not helpful</button>
                <button onClick={() => handleFeedback(m.id, 'follow_up_needed')}>🔄 Follow up</button>
              </div>
            )}
          </div>
        ))}
      </div>
      <form className="chat-input-form" onSubmit={handleSubmit}>
        <textarea
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="Describe your problem..."
          rows={2}
          disabled={sendMutation.isPending}
        />
        <button type="submit" disabled={sendMutation.isPending || !input.trim()}>
          {sendMutation.isPending ? 'Sending...' : 'Send'}
        </button>
      </form>
    </div>
  );
}
