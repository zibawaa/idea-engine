import { useState, useEffect } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { ChatTimeline } from './components/ChatTimeline';
import { TemplateSidebar } from './components/TemplateSidebar';
import { Settings } from './components/Settings';
import { useChatStore } from './store/chatStore';
import { createChat, listChats } from './services/api';
import { useApiContext } from './hooks/useApiContext';
import './App.css';

function App() {
  const queryClient = useQueryClient();
  const { currentChatId, setCurrentChatId } = useChatStore();
  const apiCtx = useApiContext();
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const [settingsOpen, setSettingsOpen] = useState(false);

  const { data: chats = [] } = useQuery({
    queryKey: ['chats', apiCtx.useCloudBackend, apiCtx.apiUrl],
    queryFn: () => listChats(apiCtx),
  });

  const createChatMutation = useMutation({
    mutationFn: (title: string) => createChat(title, undefined, apiCtx),
    onSuccess: (id) => {
      queryClient.invalidateQueries({ queryKey: ['chats'] });
      setCurrentChatId(id);
    },
  });

  useEffect(() => {
    if (chats.length === 0 && !createChatMutation.isPending && !createChatMutation.isSuccess) {
      createChatMutation.mutate('New Chat');
    }
  }, [chats.length]);

  return (
    <div className="app">
      <header className="app-header">
        <button
          className="sidebar-toggle"
          onClick={() => setSidebarOpen((o) => !o)}
          aria-label="Toggle sidebar"
        >
          {sidebarOpen ? '◀' : '▶'}
        </button>
        <h1 className="app-title">Idea Engine</h1>
        <button
          className="settings-btn"
          onClick={() => setSettingsOpen(true)}
          aria-label="Settings"
        >
          ⚙ Settings
        </button>
        <button
          className="new-chat-btn"
          onClick={() => createChatMutation.mutate('New Chat')}
          disabled={createChatMutation.isPending}
        >
          + New Chat
        </button>
      </header>

      <div className="app-body">
        {sidebarOpen && (
          <TemplateSidebar
            chats={chats}
            currentChatId={currentChatId}
            onSelectChat={setCurrentChatId}
            onNewChat={() => createChatMutation.mutate('New Chat')}
          />
        )}
        <main className="main-content">
          <ChatTimeline chatId={currentChatId} />
        </main>
      </div>
      <Settings open={settingsOpen} onClose={() => setSettingsOpen(false)} />
    </div>
  );
}

export default App;
