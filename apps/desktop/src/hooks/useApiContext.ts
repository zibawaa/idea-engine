import { useChatStore } from '../store/chatStore';

export function useApiContext() {
  const { useCloudBackend, apiUrl, apiKeys } = useChatStore();
  return {
    useCloudBackend,
    apiUrl: apiUrl || undefined,
    apiKeys,
  };
}
