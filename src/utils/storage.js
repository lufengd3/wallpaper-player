// import { appDataDir } from '@tauri-apps/api/path';
import { readTextFile, writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';
import { invoke } from '@tauri-apps/api';

const FS_OPTIONS = { dir: BaseDirectory.AppData };

export class Storage {
  constructor(storeName = '') {
    this.storeName = storeName || 'default';
    this._init_task = [];
    this.initStorageObj();
  }

  async initStorageObj() {
    const appConfStr = await readTextFile(this.storeName, FS_OPTIONS).catch(console.log);
    // this.storageObj = JSON.parse(localStorage.getItem(this.storeName) || '{}');
    this.storageObj = JSON.parse(appConfStr || '{}');
    this._init_task.forEach((fn) => {
      fn.call(this);
    });
  }

  registerInitTask(fn) {
    if (typeof fn === 'function') {
      this._init_task.push(fn);
    } else {
      console.warn('invalid task', fn);
    }
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
  autoShare: 'autoshare',
};

export const SettingStorage = new Storage('appconf');

SettingStorage.registerInitTask(() => {
  const autoShareState = SettingStorage.get(SETTING_KEYS.autoShare);
  console.log('autoshare', autoShareState);
  if (autoShareState) {
    invoke('update_autoshare_state', {
      nextState: autoShareState
    }).then((res) => {
      console.log(res);
    }).catch((e) => {
      console.error(e);
    })
  }
})
