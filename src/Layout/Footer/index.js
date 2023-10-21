import { useState, useEffect } from 'react';
import { useRecoilState } from 'recoil';
import { invoke, tauri, event } from '@tauri-apps/api';
import { wpUrlSelector } from '../../store/index';
import { wpUtils } from '../../utils/wp';
import styles from './index.module.css';

const PLAYSTATUS_KEY = 'wp:playstatus';
const PLAYSEQUENCE_KEY = 'wp:palysequence';
const SEQUENCE_TYPES = {
  default: 'default',
  random: 'random'
};

export default ({ children }) => {
  const [playStatus, setPlayStatus] = useState(localStorage.getItem(PLAYSTATUS_KEY) == 'true');
  const [playSequence, setPlaySequence] = useState(localStorage.getItem(PLAYSEQUENCE_KEY) || SEQUENCE_TYPES.default);
  const [url, setUrl] = useRecoilState(wpUrlSelector);

  useEffect(() => {
    if (playStatus) {
      wpUtils.startPlayList();
    }
    changeWpPlaySequence();
  }, []);

  const changeWpPlaySequence = () => {
    if (playSequence === SEQUENCE_TYPES.random) {
      wpUtils.randomSequence = true;
    } else {
      wpUtils.randomSequence = false;
    }
  }

  const togglePlayStatus = () => {
    const nextPlayStatus = !playStatus;
    setPlayStatus(nextPlayStatus);
    localStorage.setItem(PLAYSTATUS_KEY, nextPlayStatus);
    wpUtils.startPlayList();
  }

  const togglePlaySequence = () => {
    let nextPlaySequence = playSequence === SEQUENCE_TYPES.random ? SEQUENCE_TYPES.default : SEQUENCE_TYPES.random;
    setPlaySequence(nextPlaySequence);
    localStorage.setItem(PLAYSEQUENCE_KEY, nextPlaySequence);
    changeWpPlaySequence();
  }

  useEffect(async () => {
    invoke('get_wallpaper')
      .then(res => {
        if (res) {
          setUrl(res);
        }
      })
      .catch(e => {
        console.log('get wallpaper error', e);
      });
    
    const unlistenWpChanged = await event.listen('backend:wpchanged', ({payload}) => {
      console.log(payload?.filepath)
      setUrl(payload?.filepath);
    });

    const unlistenNextWp = await event.listen('backend:nextwp', ({payload}) => {
      wpUtils.nextWallPaper();
    });

    return () => {
      unlistenWpChanged();
      unlistenNextWp();
    }
  }, [])

  return (
    <div className={styles.footerContainer}>
      <div className={styles.left}>
        <div className={styles.previewImg}>
          {url 
          ? <img src={url} className={styles.previewImg}/>
          : <svg t="1671376342715" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="4498" width="80" height="45"><path d="M928 192H96c-17.6 0-32 14.4-32 32v576c0 17.6 14.4 32 32 32h832c17.6 0 32-14.4 32-32V224c0-17.6-14.4-32-32-32zM192 416c0-52.8 43.2-96 96-96s96 43.2 96 96-43.2 96-96 96-96-43.2-96-96z m384 288H317.6l128.8-224L520 607.2l128-221.6L832 704H576z" p-id="4499"></path></svg>
          }
        </div>
      </div>
      <div className={styles.center}>
        <div className={styles.controlIcon} onClick={wpUtils.prevWallPaper}>
          <svg t="1671371288965" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="3275" ><path d="M803.584 134.592c-18.176-10.048-39.936-10.112-58.112 0L192 443.008l0-282.88c0-17.664-14.336-32-32-32S128 142.464 128 160.128l0 344.896C127.744 507.2 127.104 509.248 127.104 511.488S127.744 515.84 128 518.016l0 346.112c0 17.664 14.336 32 32 32s32-14.336 32-32L192 579.968l553.408 308.352C754.56 893.44 764.48 896 774.528 896s19.968-2.56 29.056-7.616c18.176-10.112 28.992-28.224 28.992-48.576L832.576 183.104C832.576 162.88 821.76 144.768 803.584 134.592zM768.192 829.248 198.016 511.488 771.328 192 768.192 829.248z" p-id="3276"></path></svg>
        </div>
        <div className={styles.playIcon} onClick={togglePlayStatus}>
          {playStatus  
            ? <svg t="1671371670399" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="7029"><path d="M510.9 60.7c-245.6 0-446.7 199.8-446.7 446.7C64.2 753 263.9 954 510.8 954s446.6-199.7 446.6-446.6c0.1-245.6-199.6-446.7-446.5-446.7z m139.8 574c0 8.8-7.2 16-16 16H389.3c-8.8 0-16-7.2-16-16V389.3c0-8.8 7.2-16 16-16h245.5c8.8 0 16 7.2 16 16v245.4z" p-id="7030"></path></svg>
            : <svg t="1671371553424" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="2285"><path d="M0 512C0 229.23 229.23 0 512 0s512 229.23 512 512-229.23 512-512 512S0 794.77 0 512z m669.442 11.317c7.033-6.218 7.033-16.416 0-22.634L428.767 295.477c-3.475-3.15-7.695-4.477-11.832-4.477-8.935 0-17.374 6.301-17.374 15.753v410.494c0 9.452 8.44 15.753 17.374 15.753 4.137 0 8.357-1.41 11.832-4.477l240.675-205.206z" fill="#000000" p-id="2286"></path></svg>
          }
        </div>
        <div className={styles.controlIcon} onClick={wpUtils.nextWallPaper}>
          <svg t="1671371391108" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="5698"><path d="M832 160.128c0-17.664-14.336-32-32-32S768 142.464 768 160.128l0 282.88L214.592 134.592c-18.176-10.112-39.936-10.048-58.112 0C138.24 144.768 127.424 162.88 127.424 183.168l0 656.64c0 20.224 10.816 38.464 29.056 48.576C165.568 893.44 175.488 896 185.472 896s19.968-2.56 29.056-7.616L768 579.968l0 284.096c0 17.664 14.336 32 32 32s32-14.336 32-32L832 160.128zM191.808 829.248 188.672 192l573.312 319.488L191.808 829.248z" p-id="5699"></path></svg>
        </div>
      </div>
      <div className={styles.right}>
        <div className={styles.rightIcon} onClick={togglePlaySequence}>
          {playSequence === SEQUENCE_TYPES.default && <svg t="1671376768932" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="3178" width="20" height="20"><path d="M694.4 854.4H195.2l48 44.8c9.6 6.4 16 16 16 28.8-3.2 19.2-19.2 32-38.4 32-9.6 0-22.4-6.4-28.8-12.8l-108.8-96c-12.8-12.8-16-35.2 0-48L192 704c6.4-6.4 19.2-9.6 28.8-9.6 19.2 0 35.2 16 35.2 35.2 0 9.6-6.4 19.2-12.8 25.6l-41.6 38.4h496c112 0 198.4-89.6 198.4-198.4v-86.4c0-19.2 12.8-32 32-32s32 12.8 32 32v86.4c0 140.8-118.4 259.2-265.6 259.2zM329.6 169.6h496l-48-44.8c-9.6-6.4-16-16-16-28.8 3.2-19.2 19.2-32 38.4-32 9.6 0 22.4 6.4 28.8 12.8l108.8 96c12.8 12.8 16 35.2 0 48L832 320c-6.4 6.4-19.2 9.6-28.8 9.6-19.2 0-35.2-16-35.2-35.2 0-9.6 6.4-19.2 12.8-25.6l41.6-38.4H326.4C217.6 233.6 128 323.2 128 435.2v89.6c0 19.2-12.8 32-32 32s-32-12.8-32-32v-86.4C64 288 182.4 169.6 329.6 169.6z" p-id="3179"></path></svg>}
          {playSequence === SEQUENCE_TYPES.random && <svg t="1671376937360" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="3334" width="28" height="28"><path d="M844.8 665.6c-6.4-6.4-16-12.8-25.6-9.6-19.2 0-35.2 16-35.2 35.2 0 9.6 6.4 19.2 12.8 25.6l41.6 41.6c-44.8-6.4-86.4-22.4-121.6-51.2-3.2 0-3.2-3.2-6.4-6.4L332.8 304C268.8 233.6 192 195.2 99.2 195.2c-19.2 0-35.2 16-35.2 35.2s16 32 35.2 32c73.6 0 134.4 32 182.4 86.4l384 400 6.4 6.4c48 38.4 108.8 64 172.8 70.4l-48 44.8c-9.6 6.4-16 19.2-16 28.8 0 19.2 19.2 35.2 38.4 32 9.6 0 19.2-6.4 25.6-12.8l99.2-92.8c16-16 16-41.6 0-57.6l-99.2-102.4z m-3.2-556.8c-12.8-16-32-19.2-48-6.4-9.6 6.4-12.8 16-12.8 25.6 0 12.8 3.2 22.4 16 28.8l41.6 41.6c-73.6 9.6-140.8 38.4-192 89.6l-115.2 118.4c-12.8 12.8-12.8 32 0 44.8 6.4 6.4 16 9.6 25.6 9.6s19.2-3.2 25.6-9.6l112-118.4c41.6-38.4 92.8-64 147.2-70.4l-44.8 44.8c-6.4 6.4-12.8 16-12.8 25.6 0 19.2 16 35.2 32 35.2 9.6 0 19.2-3.2 28.8-9.6L950.4 256c12.8-12.8 12.8-35.2 0-48l-108.8-99.2m-438.4 448c-9.6 0-19.2 3.2-25.6 9.6l-118.4 121.6c-48 44.8-96 67.2-160 67.2H96c-19.2 0-35.2 16-35.2 35.2s16 32 35.2 32h3.2c83.2 0 147.2-32 211.2-86.4l121.6-124.8c6.4-6.4 9.6-12.8 9.6-22.4 0-9.6-3.2-16-9.6-22.4-9.6-6.4-19.2-9.6-28.8-9.6z" p-id="3335"></path></svg> }
        </div>
      </div>
    </div>
  );
};