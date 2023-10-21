// import { appDataDir } from '@tauri-apps/api/path';
import { readTextFile, writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';
import * as fsx from '@tauri-apps/api/fs';

const FS_OPTIONS = { dir: BaseDirectory.AppData };

export class Storage {
  constructor(storeName = '') {
    this.storeName = storeName || 'default';
    this.initStorageObj();
  }

  async initStorageObj() {
    const appConfStr = await readTextFile(this.storeName, FS_OPTIONS).catch(console.log);
    // this.storageObj = JSON.parse(localStorage.getItem(this.storeName) || '{}');
    this.storageObj = JSON.parse(appConfStr || '{}');
  }

  get(key) {
    return this.storageObj?.[key];
  }

  async set(key, val) {
    console.log('storage set ', key, val);
    this.storageObj[key] = val;
    // localStorage.setItem(this.storeName, JSON.stringify(this.storageObj));
    return await writeTextFile(this.storeName, JSON.stringify(this.storageObj), FS_OPTIONS);
  }
}

export const SETTING_KEYS ={
  imgFolder: 'img_folder',
  loopCycle: 'loopcycle',
  autoStart: 'autostart',
};

export const SettingStorage = new Storage('appconf');