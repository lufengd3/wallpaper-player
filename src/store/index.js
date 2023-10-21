import { atom, selector } from 'recoil';
import { tauri } from '@tauri-apps/api';
import { wpUtils } from '../utils/wp';

export const wpUrl = atom({
  key: 'wpUrl', // unique ID (with respect to other atoms/selectors)
  default: '', // default value (aka initial value)
});

export const wpUrlSelector = selector({
  key: 'wpUrlSelector',
  get: ({ get }) => get(wpUrl),
  set: ({ set }, newValue) => {
    let filePath;
    if (/(https|asset)?:\/\//i.test(newValue)) {
      filePath = decodeURIComponent(newValue.slice(newValue.lastIndexOf('/') + 1));
    } else {
      filePath = newValue;
      newValue = tauri.convertFileSrc(newValue);
    }
    set(wpUrl, newValue);
    wpUtils.currentImgPath = filePath;
    // updateRemoteUrl(newValue);
  }
});

export const wpKeyword = atom({
  key: 'wpKeyword',
  default: localStorage.getItem('keyword') || 'wallpaper',
});

export const wpKeywordSelector = selector({
  key: 'wpKeywordSelector',
  get: ({ get }) => get(wpKeyword),
  set: ({ set }, newValue) => {
    localStorage.setItem('keyword', newValue);
    set(wpKeyword, newValue);
  }
});