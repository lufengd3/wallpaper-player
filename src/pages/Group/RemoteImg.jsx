import { useState, useEffect, useRef } from 'react';
import { useRecoilState } from 'recoil';
import { wpUrlSelector } from '../../store/index';
import { updateWallpaper } from '../../utils/wp';
import Loading from '../../components/Loading';
import styles from './RemoteImg.module.css';

export default function Mod({src}) {
  const [iframeSrc, setIframeSrc] = useState('');
  const [ , setUrl] = useRecoilState(wpUrlSelector);

  const  handleImgError = (e) => {
    console.log('remote img error', e);
    setIframeSrc(src);
  }
  const handleClick = () => {
    // Loading.show();
    updateWallpaper(src, 'donotshare')
      .then((res) => {
        if (res) {
          // 更新全局数据
          setUrl(src);
        } else {
          throw new Error('update wallpaper error');
        }
      }).catch((e) => {
        console.log(e);
        alert(e.message);
      }).finally(() => {
        // Loading.hide();
      });
  }

  return (
    <div className={styles.imgContainer}>
      {iframeSrc 
      ? <iframe src={src} className={styles.imgIframe} referrerPolicy="no-referrer" />
      : <img 
        src={src} 
        loading="lazy"
        className={styles.imgItem}
        onError={handleImgError}
        />
      }
      <div onClick={handleClick} className={styles.mask}></div>
    </div>
  );
}