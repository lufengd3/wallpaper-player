import { useState, useEffect, useRef } from 'react';
import RemoteImg from './RemoteImg';
import { SettingStorage, SETTING_KEYS } from '../../utils/storage';
import { getImgs } from '../../utils/api';
import Loading from '../../components/Loading';
import styles from './index.module.css';

function App() {
  const [imgs, setImgs] = useState([]);
  const [authError, setAuthError] = useState(false);

  useEffect(() => {
    if (!SettingStorage.get(SETTING_KEYS.autoShare)) {
      setAuthError(true);
      return;
    }

    Loading.show();
    getImgs()
      .then((data) => {
        console.log(data);
        if (Array.isArray(data)) {
          setImgs(data);
        } else {
          alert('暂无共享图片')
        }
      }).catch(e => {
        alert(e.message);
      }).finally(Loading.hide)
  }, []);

  return (
    <div className={styles.App}>
      {authError && <div className={styles.messageBar}>
        <p>请先在 设置 页面打开<span className={styles.actionText}>自动分享</span>功能，方可查看群组共享内容</p>
      </div>}
      <div className={styles.gallery}>
        {imgs.map((item, i) => {
          const {url} = item;
          return (
            <RemoteImg 
              key={i}
              src={url} 
            />
          );
        })}
      </div>
    </div>
  );
}

export default App;