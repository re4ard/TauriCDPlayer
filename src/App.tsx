import React, { useEffect, useState } from 'react';
import WindowsPage from './WindowsPage';
import MacPage from './MacPage';
import { invoke } from '@tauri-apps/api/tauri';

const App: React.FC = () => {
  const [os, setOs] = useState<string | null>(null);

  useEffect(() => {
    // Fetch the OS information from the Tauri backend
    invoke('get_os')
      .then((os) => {
        setOs(os as string);
      })
      .catch((error) => console.error('Error getting OS:', error));
  }, []);

  if (!os) {
    return <div>Loading...</div>;
  }

  return os === 'windows' ? <WindowsPage /> : <MacPage />;
};

export default App;
