import React, { useEffect, useState } from 'react';
import WindowsDetector from './OS-CD-Detector/WindowsCD-Detector';
import MacDetector from './OS-CD-Detector/MacCD-Detector';
import LinuxDetector from './OS-CD-Detector/LinuxCD-Detector';
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

  switch (os) {
    case 'windows':
      return <WindowsDetector />;
    case 'macos':
      return <MacDetector />;
    case 'linux':
      return <LinuxDetector />;
    default:
      return <div>Unsupported OS</div>;
  }
};

export default App;
