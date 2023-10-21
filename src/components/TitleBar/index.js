import { appWindow } from '@tauri-apps/api/window'
import { useState } from 'react';
import './index.css';

const MaxIcon = () => <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="M4 4h16v16H4V4m2 4v10h12V8H6Z"/></svg>;
const RestoreIcon = () => 
<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="M4 8h4V4h12v12h-4v4H4V8m12 0v6h2V6h-8v2h6M6 12v6h8v-6H6Z"/></svg>;


export default () => {
  const [max, setMax] = useState(false);
  const toggleIcon = () => {
    setMax(!max);
  }

  return (
    <div data-tauri-drag-region className="titlebar">
      <div className="titlebar-button" id="titlebar-minimize" onClick={() => {
        appWindow.minimize();
      }}>
        <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="M20 14H4v-4h16"/></svg>
      </div>
      <div className="titlebar-button" id="titlebar-maximize" onClick={() => {
        appWindow.toggleMaximize();
        toggleIcon();
      }}>
        {max ? <RestoreIcon /> : <MaxIcon />}
      </div>
      <div className="titlebar-button" id="titlebar-close" onClick={() => {appWindow.hide()}}>
        <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12L19 6.41Z"/></svg>
      </div>
    </div>
  );
}