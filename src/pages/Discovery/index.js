import { useEffect } from 'react';
import { invoke, fs, window as tauriWindow } from '@tauri-apps/api';
import SiteIcon from './site-icon';
import { SITES_INFO } from './conf';
import './index.css';

function updateWp(imgUrl) {
  console.log('set ', imgUrl);
  invoke('update_wallpaper', {
    // url: 'https://img.alicdn.com/imgextra/i2/1993922459/TB2SyrclmMmBKNjSZTEXXasKpXa_!!1993922459.jpg_430x430q90.jpg'
    url: imgUrl
  }).then(() => {
    console.log('success');
  }).catch((e) => {
    console.error(e);
  });
}

function handleContextMenu(e) {
  e.preventDefault();
  console.log('oncontextmenu');
  if (/img/i.test(e.target.tagName)) {
    updateWp(e.target.src);
  }
}

function openWindow(name, url) {
  if (name && tauriWindow.WebviewWindow.getByLabel(name)) {
    tauriWindow.WebviewWindow.getByLabel(name).setFocus();
  } else if (url && name) {
    const webview = new tauriWindow.WebviewWindow(name, {
      url,
      width: 900,
      title: name || url,
      center: true,
    });
  }
}

const SEARCH_PLACEHOLDER = '使用图片关键词搜索，或输入网址';

export default function() {
  function handleSearch(e) {
    const text = e.target.value;
    if(e.key !== 'Enter' || !text) return;
    
    const urlPattern = /^https?:\/\/([^\/]*)/i;
    const matches = text.match(urlPattern);
    if (matches) {
      const [url, domain] = matches;
      const windowLabel = domain.replace(/\./g, '');
      openWindow(windowLabel, url);
    } else if (text) {
      const searchWindowLabel = 'unsplash_search';
      const searchWindow = tauriWindow.WebviewWindow.getByLabel(searchWindowLabel);
      if (searchWindow) {
        searchWindow.close();
      }
      openWindow(searchWindowLabel, `https://unsplash.com/s/photos/${text}`);
    }
  }

  function handleSiteClick(e) {
    const elm = e.target.closest(".site-item");
    const name = elm?.dataset?.name;
    const url = elm?.dataset?.url;
    console.log('openwindow', name, url);

    openWindow(name, url);
  }


  return (
    <div className='discovery-container'>
      <input className="search" type="text" placeholder={SEARCH_PLACEHOLDER} onKeyDown={handleSearch} />
      <div className='site-container'>
        <ul onClick={handleSiteClick}>
          {SITES_INFO.map((item, index) => {
            return <li className="site-item" key={index} data-url={item.url} data-name={item.name}>
              <div className='favicon-container'> 
                <SiteIcon data={item} />
              </div>
              <div className='site-title'>
                <span>{item.name}</span>
              </div>
              </li>
          })}
        </ul>
      </div>
    </div>
  )
}