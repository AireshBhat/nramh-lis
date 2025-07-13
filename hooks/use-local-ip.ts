import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

export function useLocalIp() {
  const [localIp, setLocalIp] = useState<string>('Loading...');
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const getLocalIp = async () => {
      try {
        const ip = await invoke('get_local_ip');
        setLocalIp(ip as string);
      } catch (error) {
        console.warn('Could not fetch local IP:', error);
        setLocalIp('192.168.1.100'); // Fallback
      } finally {
        setIsLoading(false);
      }
    };

    getLocalIp();
  }, []);

  return { localIp, isLoading };
} 