import { useEffect, useState } from 'react';
import { dialog, shell, invoke } from '@tauri-apps/api';
import * as autostart from './autostart';
import { SettingStorage, SETTING_KEYS } from '../../utils/storage';
import styles from './index.module.css';

let initialAutoStartState = false;
autostart.isEnabled().then(res => {
  initialAutoStartState = res;
}).catch(e => {
  console.log(e);
});

export default function() {
  const [folder, setFolder] = useState(SettingStorage.get(SETTING_KEYS.imgFolder) || '');
  const [loopCycle, setLoopCycle] = useState(SettingStorage.get(SETTING_KEYS.loopCycle) || '');
  const [autoShareState, setAutoShareState] = useState(SettingStorage.get(SETTING_KEYS.autoShare) || false);
  const [autoStartState, setAutoStartState] = useState(initialAutoStartState);
  useEffect(() => {
    autostart.isEnabled().then(res => {
      setAutoStartState(res);
      initialAutoStartState = res;
    }).catch(e => {
      console.log(e);
    });
  }, []);

  function selectDirectory() {
    dialog.open({
      directory: true
    }).then((res) => {
      if (res) {
        setFolder(res);
        SettingStorage.set(SETTING_KEYS.imgFolder, res).catch((e) => {
          alert(e.message || e);
        });
      }
    }).catch((e) => {
      console.error(e);
    })
  }

  async function openFolder() {
    await shell.open(folder).catch((e) => {
      alert(JSON.stringify(e));
    });
  }

  function changeCycle(e) {
    console.log(e.target.value)
    setLoopCycle(e.target.value);
    SettingStorage.set(SETTING_KEYS.loopCycle, e.target.value).catch(e => {
      alert(e.message || e);
    })
  }

  function handleAutoStart(e) {
    let action = Promise.resolve();
    if (e.target.checked) {
      action = autostart.enable;
    } else {
      action = autostart.disable;
    }

    action().then(res => {
      initialAutoStartState = e.target.checked;
    }).catch(e => {
      console.log(e);
    });
  }

  function handleAutoShare(e) {
    const nextState = e.target.checked;
    SettingStorage.set(SETTING_KEYS.autoShare, nextState)
    .then(() => {
      invoke('update_autoshare_state', {
        nextState
      })
    }).catch(e => {
      alert(e.message || e);
    })
  }

  return (
    <div className='setting-container'>
      <fluent-card class={styles.settingItem}>
        <div>图片保存位置</div>
        <div className={styles.folderSelectorContainer  }>
          {/* https://github.com/microsoft/fast/tree/master/packages/web-components/fast-foundation/src/text-field */}
          <fluent-text-field size={32} value={folder} onClick={selectDirectory} >
          </fluent-text-field>
          {/* <div className={styles.folderSelector}>{folder}</div> */}
          <svg onClick={openFolder} className={styles.folderIcon} xmlns="http://www.w3.org/2000/svg" width="28" height="28"><path fill="#777" d="M4 22.4q-.825 0-1.413-.587T2 20.4v-12q0-.825.588-1.413T4 6.4h6l2 2h8q.825 0 1.413.588T22 10.4H11.175l-2-2H4v12l2.4-8h17.1l-2.575 8.575q-.2.65-.738 1.038T19 22.4H4Zm2.1-2H19l1.8-6H7.9l-1.8 6Zm0 0l1.8-6l-1.8 6ZM4 10.4v-2v2Z"/></svg>
        </div>
      </fluent-card>

      <fluent-card class={styles.settingItem}>
        <div>切换间隔</div>
        <fluent-select value={loopCycle} onInput={changeCycle}>
          <fluent-option value="5"> 5min </fluent-option>
          <fluent-option value="15"> 15min </fluent-option>
          <fluent-option value="30"> 30min </fluent-option>
          <fluent-option value="60"> 1hour </fluent-option>
        </fluent-select>
      </fluent-card>

      <fluent-card class={styles.settingItem}>
        <div>开机启动</div>
        <fluent-switch checked={autoStartState} onClick={handleAutoStart} />
      </fluent-card>

      <fluent-card class={styles.settingItem}>
        <div>自动分享：下载壁纸时自动分享到群组</div>
        <fluent-switch checked={autoShareState} onClick={handleAutoShare} />
      </fluent-card>
      
      <fluent-card class={styles.settingItem}>
        <div>帮助</div>
        <div>
          <a target='_blank' href="https://github.com/lufengd3/wallpaper-player">
            https://github.com/lufengd3/wallpaper-player
          </a>
        </div>
      </fluent-card>
    </div>
  );

}