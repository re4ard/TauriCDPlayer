import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

function MetadataExtractorPage() {
  const [status, setStatus] = useState('');
  const [drive, setDrive] = useState('');

  useEffect(() => {
    const extractMetadata = async () => {
      try {
        // Find the CD drive letter
        const cdDrive = await invoke('find_cd_drive') as string;
        if (!cdDrive) {
          setStatus('No CD drive found.');
          return;
        }
        setDrive(cdDrive);

        // Define output path
        const outputPath = `src/Tracks-Art/metadata.json`; // Adjust the output path

        // Generate metadata JSON
        await invoke('generate_metadata_json', { drive: cdDrive, outputPath });
        setStatus('Metadata extraction successful!');
      } catch (error) {
        console.error('Error extracting metadata:', error);
        setStatus(`Failed to extract metadata: ${error}`);
      }
    };

    extractMetadata();
  }, []); // Empty dependency array ensures this runs only once after initial render

  return (
    <div>
      <h1>Metadata Extractor</h1>
      <p>Drive: {drive}</p>
      <p>Status: {status}</p>
    </div>
  );
}

export default MetadataExtractorPage;
