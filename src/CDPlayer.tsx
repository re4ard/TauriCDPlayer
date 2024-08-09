import React from 'react';
import { appCacheDir } from '@tauri-apps/api/path';
const appCacheDirPath = await appCacheDir();

const MacPage: React.FC = () => {
  console.log(appCacheDirPath)
  return (
    <div>
      <h1>CD PLAYER</h1>
      
    </div>
  );
};

export default MacPage;
