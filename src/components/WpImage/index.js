import { useState } from 'react';
import { useRecoilState } from 'recoil';
import { removeFile } from '@tauri-apps/api/fs';
import { updateWallpaper } from '../../utils/wp';
import { wpUrlSelector } from '../../store/index';
import Loading from '../Loading';
import styles from './index.module.css'

export default ({src, imgUrl, thumbnailSrc, thumbnailPath, style}) => {
  const [imgSrc, setImgSrc] = useState(thumbnailSrc || src);
  const [ , setUrl] = useRecoilState(wpUrlSelector);
  const [imgInfo, setImgInfo] = useState({});
  const [removed, setRemoved] = useState(false);
  const [resolution, setResulution] = useState('');

  function handleClick() {
    Loading.show();
    updateWallpaper(imgUrl)
      .then((res) => {
        if (res) {
          // 更新全局数据
          setUrl(imgUrl);
        } else {
          throw new Error('update wallpaper error');
        }
      }).catch((e) => {
        alert(e.message);
      }).finally(() => {
        Loading.hide();
      });
  }

  function handleLoad({target: img}) {
    // const {naturalHeight, naturalWidth} = img;
    // setImgInfo(img);

    // if (naturalWidth >= 7680) {
    //   setResulution('8K');
    // } else if (naturalWidth >= 3840) {
    //   setResulution('4K');
    // } else if (naturalWidth >= 2560) {
    //   setResulution('2K');
    // } else if (naturalWidth >= 1920) {
    //   setResulution('1080p')
    // } else {
    //   setResulution('');
    // }
  }

  function handleError() {
    if (imgSrc === thumbnailSrc) {
      setImgSrc(src);
    }
  }

  async function deleteImg() {
    setRemoved(true);
    await removeFile(imgUrl).catch((e) => {
      alert(JSON.stringify(e));
      setRemoved(false);
    });

    thumbnailPath && removeFile(thumbnailPath).catch((e) => {
      alert(JSON.stringify(e));
      setRemoved(false);
    });
  }

  if (removed) {
    return null;
  }

  return (
    <div className={styles.imgContainer}>
      <img 
        title="设为壁纸"
        src={imgSrc}
        className={style}
        loading="lazy"
        onClick={handleClick}
        onLoad={handleLoad}
        onError={handleError}
      />
      <div className={styles.footer}>
        {/* <span>{imgInfo.naturalWidth} x {imgInfo.naturalHeight}</span> */}
        <span>{resolution}</span>
        <svg onClick={deleteImg} className={styles.deleteIcon} xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 2.4 24 24"><path fill="#777" d="M7 23.4q-.825 0-1.413-.587T5 21.4v-13q-.425 0-.713-.288T4 7.4q0-.425.288-.713T5 6.4h4q0-.425.288-.713T10 5.4h4q.425 0 .713.288T15 6.4h4q.425 0 .713.288T20 7.4q0 .425-.288.713T19 8.4v13q0 .825-.588 1.413T17 23.4H7Zm0-15v13h10v-13H7Zm2 10q0 .425.288.713T10 19.4q.425 0 .713-.287T11 18.4v-7q0-.425-.288-.713T10 10.4q-.425 0-.713.288T9 11.4v7Zm4 0q0 .425.288.713T14 19.4q.425 0 .713-.287T15 18.4v-7q0-.425-.288-.713T14 10.4q-.425 0-.713.288T13 11.4v7Zm-6-10v13v-13Z"/></svg>
      </div>
    </div>
  );
}