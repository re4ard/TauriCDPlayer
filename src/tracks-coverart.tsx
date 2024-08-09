import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import CDPLAYER from './CDPlayer';

function WindowsPage() {
  const [cdInserted, setCdInserted] = useState(false);
  const [status, setStatus] = useState('');
  const [drive, setDrive] = useState('');
  const [metadataExtracted, setMetadataExtracted] = useState(false);
  const [PathOutput, setOutputPath] = useState('');

  useEffect(() => {
    const fetchCurrentDir = async () => {
      try {
        const currentDir = await invoke<string>('get_current_dir');
        const outputPath = `${currentDir}/src/Tracks-Art/metadata.json`;
        setOutputPath(outputPath);
        console.log(outputPath); // Log the path to verify
      } catch (error) {
        console.error('Error getting current directory:', error);
      }
    };

    fetchCurrentDir();
  }, []);

  useEffect(() => {
    const checkCd = async () => {
      try {
        const cdDrive = await invoke<string>('check_cd_inserted');
        if (cdDrive) {
          setCdInserted(true);
          setDrive(cdDrive);
          setStatus('CD Detected with files');

          if (PathOutput) {
            await invoke('generate_metadata_json', { drive: cdDrive, outputPath: PathOutput });
            setStatus('Metadata extraction successful!');
            setMetadataExtracted(true);
          } else {
            setStatus('Output path not set');
          }
        } else {
          setCdInserted(false);
          setStatus('No CD detected');
        }
      } catch (error) {
        console.error('Error checking CD:', error);
        setStatus(`Failed to check CD: ${error}`);
      }
    };

    checkCd();
  }, [PathOutput]);

  if (metadataExtracted) {
    return <CDPLAYER />;
  }

  return (
    <div>
      <h1>Metadata Extractor</h1>
      <p>Drive: {drive}</p>
      <p>Status: {status}</p>
    </div>
  );
}

export default WindowsPage;
