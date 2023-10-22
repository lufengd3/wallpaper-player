import { useState } from 'react';
import { getSites } from '../../utils/api';
import { useEffect } from 'react';

let SITES_INFO = [{
  url: 'https://unsplash.com/t/wallpapers',
  name: 'unsplash',
  icon: ''
}, {
  url: 'https://wallhaven.cc/',
  name: 'wallhaven',
  icon: ''
}, {
  url: 'https://www.wallpaperhub.app/',
  name: 'wallpaperhub',
  icon: ''
}, {
  url: 'https://www.douban.com/group/634436/',
  name: '豆瓣',
  icon: ''
}, {
  url: 'https://www.dpm.org.cn/lights/royal.html',
  name: '故宫',
  icon: ''
}, {
  url: 'https://wall.alphacoders.com/',
  name: 'Abyss',
  icon: ''
}, {
  url: 'https://gikken.co/artpaper/collections/',
  name: 'artpaper',
  icon: ''
}, {
//   url: 'https://anime.goodfon.com/',
//   name: 'goodfon',
//   icon: ''
// }, {
  url: 'https://replicate.com/stability-ai/stable-diffusion',
  name: 'AIGC',
  icon: ''
}, {
  url: 'https://www.pexels.com/',
  name: 'pexels',
  icon: ''
}, {
  url: 'https://www.lifeofpix.com/',
  name: 'lifeofpix',
  icon: ''
}];

let lastReqTime = 0;

export function useSites() {
  const [sites, setSites] = useState(SITES_INFO);
  
  useEffect(() => {
    if ((Date.now() - lastReqTime) < 10e3 * 60 * 60) {
      return;
    }

    lastReqTime = Date.now();
    getSites().then((res) => {
      console.log(res);
      if (Array.isArray(res)) {
        setSites(res);
        SITES_INFO = res;
      }
    }).catch((e) => {
      console.error(e);
    });
  }, []);

  return sites;
}