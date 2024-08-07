import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import Convert from '../tracks-coverart'

function WindowsPage() {
  const [cdInserted, setCdInserted] = useState(false);

  useEffect(() => {
    const checkCd = async () => {
      try {
        const inserted = await invoke<boolean>('check_cd_inserted');
        console.log('CD inserted:', inserted); // Log the result
        setCdInserted(inserted);
      } catch (error) {
        console.error('Error checking CD:', error);
      }
    };

    checkCd(); // Initial check
    const intervalId = setInterval(checkCd, 2000); // Check every 5 seconds

    return () => clearInterval(intervalId); // Cleanup interval on component unmount
  }, []);

  return (
    <div>
      {cdInserted ? (
        <Convert />
      ) : (
        <div>No CD detected</div>
      )}
    </div>
  );
}

export default WindowsPage;
