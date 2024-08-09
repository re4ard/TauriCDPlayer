import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import Convert from '../tracks-coverart';
import Test from '../CDNotInserted';

function WindowsPage() {
  const [cdInserted, setCdInserted] = useState(false);
  const [status, setStatus] = useState('');
  const [showTest, setShowTest] = useState(false); // Add state for showing Test component

  useEffect(() => {
    const checkCd = async () => {
      try {
        const drive = await invoke<string>('check_cd_inserted');
        console.log('CD inserted:', drive); // Log the result
        if (drive) {
          setCdInserted(true);
          setStatus('CD Detected with files');
          setShowTest(false); // Hide Test component if CD is detected
        } else {
          setCdInserted(false);
          setStatus('No CD detected');
          setShowTest(true); // Show Test component if no CD detected
        }
      } catch (error) {
        console.error('Error checking CD:', error);
        setStatus(`Error: ${error}`);
        setShowTest(true); // Show Test component if there's an error
      }
    };

    checkCd(); // Initial check
    const intervalId = setInterval(checkCd, 1000); // Check every 1 second

    return () => clearInterval(intervalId); // Cleanup interval on component unmount
  }, []); // Empty dependency array ensures this runs only once after initial render

  return (
    <div>
      {showTest ? (
        <Test />
      ) : cdInserted ? (
        <Convert />
      ) : (
        <div>{status}</div> // Display status when neither Test nor Convert is shown
      )}
    </div>
  );
}

export default WindowsPage;
