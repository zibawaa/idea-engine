import './TemplateSidebar.css';

export interface ChatItem {
  id: string;
  title: string;
  templateId?: string | null;
  createdAt?: string;
  updatedAt?: string;
}

interface TemplateSidebarProps {
  chats: ChatItem[];
  currentChatId: string | null;
  onSelectChat: (id: string | null) => void;
  onNewChat: () => void;
}

const TEMPLATES = [
  {
    id: 'youtube-playlist',
    name: 'YouTube Playlist Auto Translate',
    description: 'Plan for translating playlist videos using YouTube Data API',
  },
  {
    id: 'generic',
    name: 'Generic Idea',
    description: 'Open-ended brainstorming',
  },
];

export function TemplateSidebar({
  chats,
  currentChatId,
  onSelectChat,
  onNewChat,
}: TemplateSidebarProps) {
  return (
    <aside className="template-sidebar">
      <section className="sidebar-section">
        <h3>Chats</h3>
        <button className="new-chat-sidebar" onClick={onNewChat}>
          + New Chat
        </button>
        <ul className="chat-list">
          {chats.map((c) => (
            <li key={c.id}>
              <button
                className={`chat-item ${currentChatId === c.id ? 'active' : ''}`}
                onClick={() => onSelectChat(c.id)}
              >
                {c.title || 'Untitled'}
              </button>
            </li>
          ))}
        </ul>
      </section>
      <section className="sidebar-section">
        <h3>Templates</h3>
        <ul className="template-list">
          {TEMPLATES.map((t) => (
            <li key={t.id}>
              <button
                className="template-item"
                onClick={() => onSelectChat(null)}
              >
                <strong>{t.name}</strong>
                <span className="template-desc">{t.description}</span>
              </button>
            </li>
          ))}
        </ul>
      </section>
    </aside>
  );
}
