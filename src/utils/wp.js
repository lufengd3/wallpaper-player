import { invoke } from '@tauri-apps/api';
import { SettingStorage, SETTING_KEYS} from './storage';

export async function updateWallpaper(imgUrl, shareType = '') {
  return invoke('update_wallpaper', {
    // url: 'https://img.alicdn.com/imgextra/i2/1993922459/TB2SyrclmMmBKNjSZTEXXasKpXa_!!1993922459.jpg_430x430q90.jpg'
    url: imgUrl,
    base64Img: '',
    shareType
  })
}
class WPUtils {
  constructor() {
    this.localImagesStr = localStorage.getItem('imgs');
    this._IMG_CACHE = JSON.parse(this.localImagesStr || `[]`);
    this._currentImgPath = null;
    this._lastImgPath = null;
    this.loopHandler = null;
    this.randomSequence = false;
  }

  set IMG_CACHE(v) {
    if (Array.isArray(v)) {
      this._IMG_CACHE = v;
      const imgsStr = JSON.stringify(v);
      console.log(imgsStr.length);
      if (imgsStr !== this.localImagesStr) {
        localStorage.setItem('imgs', imgsStr);
        this.localImagesStr = imgsStr;
      }
    } else {
      console.error('Invalid IMG_CACHE value', v);
    }
  }

  get IMG_CACHE() {
    return this._IMG_CACHE || []
  }

  set currentImgPath(imgPath) {
    if (this.currentImgPath) {
      this.lastImgPath = this.currentImgPath;
    }
    this._currentImgPath = imgPath;
  }

  get currentImgPath() {
    return this._currentImgPath || null;
  }

  set lastImgPath(imgPath) {
    this._lastImgPath = imgPath;
  }

  get lastImgPath() {
    return this._lastImgPath;
  }

  startPlayList = () => {
    const loopCycle = SettingStorage.get(SETTING_KEYS.loopCycle) || '15';  // 分钟
    this.loopHandler = setInterval(this.nextWallPaper, loopCycle * 60 * 1000);
  }

  stopPlayList = () => {
    if (this.loopHandler) {
      clearInterval(this.loopHandler);
    }
  }
 
  nextWallPaper = () => {
    let currentIndex = this.IMG_CACHE.findIndex(item => {
      return item.path === this.currentImgPath;
    });
    currentIndex = currentIndex === -1 ? 0 : currentIndex;

    let nextIndex;
    console.log(this.randomSequence)
    if (this.randomSequence) {
      nextIndex = Math.round(Math.random() * (this.IMG_CACHE.length - 1));
      if (nextIndex === currentIndex && currentIndex !== 0) {
        nextIndex -= 1;
      }
    } else {
      nextIndex = currentIndex + 1 === this.IMG_CACHE.length ? 0 : currentIndex + 1;
    }
    const nextImg = this.IMG_CACHE[nextIndex];
    this.updateWallpaper(nextImg.path).catch(e => {
      console.error(e);
    });
  }

  prevWallPaper = () => {
    let lastIndex;
    if (this.lastImgPath) {
      const index = this.IMG_CACHE.findIndex(item => {
        return item.path === this.lastImgPath;
      });
      lastIndex = index === -1 ? lastIndex : index;
    } else {
      let currentIndex = this.IMG_CACHE.findIndex(item => {
        return item.path === this.currentImgPath;
      });
      currentIndex = currentIndex === -1 ? 0 : currentIndex;
      lastIndex = currentIndex === 0 ? this.IMG_CACHE.length - 1 : currentIndex - 1;
    }
    const lastImg = this.IMG_CACHE[lastIndex];
    this.updateWallpaper(lastImg.path).catch(e => {
      console.error(e);
    });
  }

  async updateWallpaper(imgUrl) {
    return invoke('update_wallpaper', {
      url: imgUrl
    });
  }
}

export const wpUtils = new WPUtils();