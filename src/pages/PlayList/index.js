// https://developer.microsoft.com/en-us/fluentui#/controls/web/detailslist

import { useState, useEffect, useRef, useMemo } from 'react';
import { fs, tauri, path } from '@tauri-apps/api';

import WpImage from '../../components/WpImage';
import { SettingStorage, SETTING_KEYS } from '../../utils/storage';
import styles from './index.module.css';
import { wpUtils } from '../../utils/wp';

function joinPath(baseDir, ...filePath) {
  if (!baseDir) return '';

  // TODO: macos linux
  const isWin = /windows/i.test(navigator.userAgent);
  const seperator = isWin ? '\\' : '/';
  const regex = new RegExp(`^\\${seperator}|\\${seperator}$`, 'g');
  let fullPath = baseDir.replace(regex, '');
  filePath.forEach((item) => {
    item = item.replace(regex, '');
    fullPath += `${seperator}${item}`;
  });

  return fullPath;
}

let imgFolder;
let thumbnailFolder;

function App() {
  const [imgs, setImgs] = useState([]);
  // const dataGridRef = useRef(null);

  useEffect(() => {
    // 放这里异步取，避免 setting storage 还没初始化好
    imgFolder = SettingStorage.get(SETTING_KEYS.imgFolder);
    thumbnailFolder = joinPath(imgFolder, 'thumbnail');

    if (imgFolder) {
      fs.readDir(imgFolder)
        .then((res = []) => {
          const localImgaes = res.filter(item => /\.(jpg|png|jpeg)$/i.test(item.name)).reverse();

          setImgs(localImgaes);
          wpUtils.IMG_CACHE = localImgaes;
          // dataGridRef.current.rowsData = res;
          if (localImgaes.length === 0) {
            alert('暂无壁纸，快去冲浪下载吧')
          }
        })
        .catch(e => {
          console.error(e);
        });
    } else {
      alert('请先在设置中选择图片保存位置');
    }
  }, []);

  return (
    <div className={styles.container}>
      {/* <fluent-data-grid ref={dataGridRef} rows-data={imgs} /> */}
      {imgs.map((item, i) => {
        const tauirFilePath = tauri.convertFileSrc(item.path);
        const fileName = item.path.replace(imgFolder, '');
        const thumbnailPath = joinPath(thumbnailFolder, fileName);
        const thumbnailSrc = tauri.convertFileSrc(thumbnailPath);
        return (
          <WpImage 
            key={i}
            src={tauirFilePath} 
            thumbnailSrc={thumbnailSrc}
            thumbnailPath={thumbnailPath}
            imgUrl={item.path} 
            style={styles.imgItem}
          />
        );
      })}
    </div>
  );
}

export default App;